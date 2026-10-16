use bevy::prelude::*;

use rand::RngExt;

use super::{AppState, GameState, GlobalRng};
use crate::consts::{START_CASH, START_PORTFOLIO_SIZE};
use crate::jokers::Joker;
use crate::stocks::{Market, Stock, Ticker};

pub struct PortfolioPlugin;

impl Plugin for PortfolioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Portfolio>()
            .add_systems(Update, (resize_portfolio, update_invested).chain())
            .add_systems(OnExit(AppState::GameOver), reset_portfolio);
    }
}

#[derive(Clone)]
pub struct Holding {
    pub ticker: Ticker,
    pub shares: f64,
    pub avg_cost: f64,
}

impl Holding {
    pub fn new(ticker: Ticker) -> Self {
        Self { ticker, shares: 0.0, avg_cost: 0.0 }
    }

    pub fn increment(&mut self, price: f64) {
        let total_cost = self.avg_cost * self.shares + price;
        self.shares += 1.0;
        self.avg_cost = total_cost / self.shares;
    }

    pub fn increment_for_free(&mut self) {
        self.increment(0.0);
    }

    pub fn decrement(&mut self) {
        self.shares -= 1.0;
    }
    pub fn div_claim(&self, market: &Market) -> f64 {
        market.get(self.ticker).div_per_share() as f64 * self.shares as f64
    }
}

#[derive(Resource)]
pub struct Portfolio {
    pub cash: f64,
    pub jokers: Vec<Joker>,
    pub max_size: usize,
    pub slots: Vec<Option<Holding>>,
}

impl Default for Portfolio {
    fn default() -> Self {
        Self {
            cash: START_CASH,
            jokers: Vec::new(),
            max_size: START_PORTFOLIO_SIZE,
            slots: vec![None; START_PORTFOLIO_SIZE],
        }
    }
}

impl Portfolio {
    pub fn slot_of(&self, ticker: Ticker) -> Option<usize> {
        self.slots.iter().position(|s| s.as_ref().is_some_and(|h| h.ticker == ticker))
    }

    pub fn holdings(&self) -> Vec<Holding> {
        self.slots.iter().flatten().cloned().collect()
    }

    /// What one share costs including the commission.
    pub fn buy_cost(stock: &Stock, commission: f64) -> f64 {
        stock.price() as f64 * (1.0 + commission)
    }

    pub fn buy(&mut self, stock: &Stock, commission: f64) -> bool {
        let price = Self::buy_cost(stock, commission);
        if self.cash < price {
            return false;
        }
        let free_slot = || self.slots.iter().position(Option::is_none);
        let Some(slot) = self.slot_of(stock.ticker).or_else(free_slot) else {
            return false;
        };
        self.slots[slot].get_or_insert_with(|| Holding::new(stock.ticker)).increment(price);
        self.cash -= price;
        true
    }

    pub fn sell(&mut self, stock: &Stock) -> bool {
        let Some(slot) = self.slot_of(stock.ticker) else {
            return false;
        };
        let Some(holding) = &mut self.slots[slot] else {
            return false;
        };
        holding.decrement();
        if holding.shares <= 0.0 {
            self.slots[slot] = None;
        }
        self.cash += stock.price() as f64;
        true
    }

    pub fn move_joker(&mut self, from: usize, to: usize) {
        if from >= self.jokers.len() {
            return;
        }
        let joker = self.jokers.remove(from);
        self.jokers.insert(to.min(self.jokers.len()), joker);
        self.renumber_jokers();
    }

    pub fn add_joker(&mut self, joker: Joker) {
        self.jokers.push(joker);
        self.renumber_jokers();
    }

    pub fn remove_joker(&mut self, index: usize) -> Joker {
        let mut joker = self.jokers.remove(index);
        self.renumber_jokers();
        joker.slot = Joker::NOT_OWNED;
        joker
    }

    fn renumber_jokers(&mut self) {
        for (slot, joker) in self.jokers.iter_mut().enumerate() {
            joker.slot = slot as i32;
        }
    }

    pub fn rnd_holding(&mut self, rng: &mut GlobalRng) -> Option<&mut Holding> {
        let held: Vec<&mut Holding> = self.slots.iter_mut().flatten().collect();
        let count = held.len();
        if count == 0 {
            return None;
        }
        held.into_iter().nth(rng.random_range(0..count))
    }
}

fn resize_portfolio(mut portfolio: ResMut<Portfolio>) {
    let wanted = portfolio.max_size;
    if portfolio.slots.len() < wanted {
        portfolio.slots.resize(wanted, None);
    }
}

fn update_invested(portfolio: Res<Portfolio>, market: Res<Market>, mut state: ResMut<GameState>) {
    let total: f64 = portfolio
        .slots
        .iter()
        .flatten()
        .map(|h| h.shares * market.get(h.ticker).price() as f64)
        .sum();
    if state.invested_euro != total {
        state.invested_euro = total;
    }
}

fn reset_portfolio(mut portfolio: ResMut<Portfolio>) {
    *portfolio = Portfolio::default();
}
