use bevy::prelude::*;

use rand::RngExt;

use super::{AppState, GameState, GlobalRng};
use crate::consts::{MIN_SHARES, START_CASH, START_PORTFOLIO_SIZE};
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
    pub fn add_free_share(&mut self) {
        let total_cost = self.avg_cost * self.shares;
        self.shares += 1.0;
        self.avg_cost = total_cost / self.shares;
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
    pub fn buy(&mut self, stock: &Stock) -> bool {
        let price = stock.price() as f64;
        if self.cash < price {
            return false;
        }
        let slots = &mut self.slots;
        let existing = slots
            .iter()
            .position(|s| s.as_ref().is_some_and(|h| h.ticker == stock.ticker));
        let Some(index) = existing.or_else(|| slots.iter().position(Option::is_none)) else {
            return false;
        };
        let holding = slots[index].get_or_insert(Holding {
            ticker: stock.ticker,
            shares: 0.0,
            avg_cost: 0.0,
        });
        let total_cost = holding.avg_cost * holding.shares + price;
        holding.shares += 1.0;
        holding.avg_cost = total_cost / holding.shares;
        self.cash -= price;
        true
    }

    pub fn move_joker(&mut self, from: usize, to: usize) {
        if from >= self.jokers.len() {
            return;
        }
        let joker = self.jokers.remove(from);
        self.jokers.insert(to.min(self.jokers.len()), joker);
    }

    pub fn sell_one(&mut self, stock: &Stock) -> bool {
        let shares = self
            .slots
            .iter()
            .flatten()
            .find(|h| h.ticker == stock.ticker)
            .map_or(0.0, |h| h.shares);
        shares > 0.0 && self.sell(stock, (1.0 / shares).min(1.0))
    }

    pub fn sell(&mut self, stock: &Stock, fraction: f64) -> bool {
        let slots = &mut self.slots;
        let Some(slot) = slots
            .iter()
            .position(|s| s.as_ref().is_some_and(|h| h.ticker == stock.ticker))
        else {
            return false;
        };
        let Some(holding) = &mut slots[slot] else {
            return false;
        };
        let sold = holding.shares * fraction.clamp(0.0, 1.0);
        holding.shares -= sold;
        self.cash += sold * stock.price() as f64;
        if holding.shares < MIN_SHARES {
            slots[slot] = None;
        }
        true
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
