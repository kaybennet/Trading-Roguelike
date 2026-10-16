use std::collections::VecDeque;

use bevy::prelude::*;
use rand::RngExt;

use super::stocks::{STOCKS, Stock, Ticker};
use crate::consts::{MARKET_HISTORY_WEEKS, STOCK_MIN_PRICE};
use crate::general::GlobalRng;

#[derive(Resource, Default)]
pub struct Market {
    pub stocks: Vec<Stock>,
}

impl Market {
    pub fn new(rng: &mut GlobalRng) -> Self {
        let stocks = STOCKS
            .iter()
            .enumerate()
            .map(|(idx, &(ticker, country, _, volatility, dividend_yield))| {
                let history = Market::generate_history(idx, rng);
                Stock { ticker, country, volatility, dividend_yield, history }
            })
            .collect();
        Self { stocks }
    }

    pub fn generate_history(idx: usize, rng: &mut GlobalRng) -> VecDeque<f32> {
        let (_, _, price, volatility, _) = STOCKS[idx];
        let backwards = std::iter::successors(Some(price), |&newer| {
            let change = rng.random_range(-1.0..1.0) * volatility;
            Some((newer / (1.0 + change)).max(STOCK_MIN_PRICE))
        });
        backwards.take(MARKET_HISTORY_WEEKS).collect::<Vec<_>>().into_iter().rev().collect()
    }

    pub fn get(&self, ticker: Ticker) -> &Stock {
        self.stocks.iter().find(|s| s.ticker == ticker).unwrap()
    }

    pub fn get_mut(&mut self, ticker: Ticker) -> &mut Stock {
        self.stocks.iter_mut().find(|s| s.ticker == ticker).unwrap()
    }

    pub fn advance_week(&mut self, rng: &mut GlobalRng) {
        for stock in &mut self.stocks {
            let change = rng.random_range(-1.0..1.0) * stock.volatility;
            stock.record((stock.price() * (1.0 + change)).max(STOCK_MIN_PRICE));
        }
    }
    pub fn create(mut commands: Commands, mut rng: ResMut<GlobalRng>) {
        commands.insert_resource(Market::new(&mut rng));
    }
}
