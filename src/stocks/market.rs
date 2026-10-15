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
            .map(|&(ticker, country, price, volatility, dividend_yield)| {
                let mut history = VecDeque::with_capacity(MARKET_HISTORY_WEEKS);
                history.push_front(price);
                for _ in 1..MARKET_HISTORY_WEEKS {
                    let change = rng.random_range(-1.0..1.0) * volatility;
                    history.push_front((history[0] / (1.0 + change)).max(STOCK_MIN_PRICE));
                }
                Stock {
                    ticker,
                    country,
                    volatility,
                    dividend_yield,
                    history,
                }
            })
            .collect();
        Self { stocks }
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
}

pub(super) fn create_market(mut commands: Commands, mut rng: ResMut<GlobalRng>) {
    commands.insert_resource(Market::new(&mut rng));
}
