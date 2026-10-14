use std::collections::VecDeque;

use bevy::prelude::*;
use rand::RngExt;

use crate::consts::{MARKET_HISTORY_WEEKS, STOCK_MIN_PRICE};
use crate::general::GlobalRng;

#[derive(Resource, Default)]
pub struct Market {
    pub stocks: Vec<Stock>,
}

pub struct Stock {
    pub symbol: String,
    pub volatility: f32,
    pub dividend_yield: f32,
    pub history: VecDeque<f32>,
}

impl Stock {
    pub fn price(&self) -> f32 {
        self.history.back().copied().unwrap_or(STOCK_MIN_PRICE)
    }

    pub fn dividend_per_share(&self) -> f32 {
        self.price() * self.dividend_yield
    }

    pub fn weekly_change_pct(&self) -> f32 {
        let mut newest_first = self.history.iter().rev();
        match (newest_first.next(), newest_first.next()) {
            (Some(now), Some(before)) => (now / before - 1.0) * 100.0,
            _ => 0.0,
        }
    }

    /// Multiplies the current price by `factor`.
    pub fn scale_price(&mut self, factor: f32) {
        if let Some(price) = self.history.back_mut() {
            *price = (*price * factor).max(STOCK_MIN_PRICE);
        }
    }

    fn record(&mut self, price: f32) {
        self.history.push_back(price);
        while self.history.len() > MARKET_HISTORY_WEEKS {
            self.history.pop_front();
        }
    }
}

const STOCKS: [(&str, f32, f32, f32); 12] = [
    ("ACME", 100.0, 0.03, 0.004),
    ("BLNK", 12.0, 0.08, 0.0),
    ("CRPT", 250.0, 0.10, 0.0),
    ("DYNO", 45.0, 0.04, 0.002),
    ("ECHO", 80.0, 0.02, 0.006),
    ("FLUX", 30.0, 0.06, 0.0),
    ("GRID", 150.0, 0.03, 0.005),
    ("HELX", 60.0, 0.05, 0.002),
    ("IONX", 20.0, 0.09, 0.0),
    ("JOLT", 95.0, 0.04, 0.003),
    ("KRNL", 180.0, 0.02, 0.007),
    ("LUMA", 8.0, 0.12, 0.0),
];

impl Market {
    pub fn new(rng: &mut GlobalRng) -> Self {
        let stocks = STOCKS
            .iter()
            .map(|&(symbol, price, volatility, dividend_yield)| {
                let mut history = VecDeque::with_capacity(MARKET_HISTORY_WEEKS);
                history.push_front(price);
                for _ in 1..MARKET_HISTORY_WEEKS {
                    let change = rng.random_range(-1.0..1.0) * volatility;
                    history.push_front((history[0] / (1.0 + change)).max(STOCK_MIN_PRICE));
                }
                Stock {
                    symbol: symbol.to_string(),
                    volatility,
                    dividend_yield,
                    history,
                }
            })
            .collect();
        Self { stocks }
    }

    pub fn get(&self, symbol: &str) -> Option<&Stock> {
        self.stocks.iter().find(|s| s.symbol == symbol)
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
