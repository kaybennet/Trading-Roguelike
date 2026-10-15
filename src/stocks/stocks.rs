use std::collections::VecDeque;

use strum::{Display, EnumIter, IntoStaticStr};

use crate::{
    consts::{MARKET_HISTORY_WEEKS, STOCK_MIN_PRICE},
    general::Boost,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumIter, IntoStaticStr, Display)]
pub enum Country {
    #[strum(serialize = "US")]
    UnitedStates,
    #[strum(serialize = "GER")]
    Germany,
    #[strum(serialize = "JP")]
    Japan,
    #[strum(serialize = "UK")]
    UnitedKingdom,
    #[strum(serialize = "FR")]
    France,
    #[strum(serialize = "CH")]
    China,
}

/// The ticker symbols of the stocks on the market.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, EnumIter, IntoStaticStr, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Ticker {
    #[default]
    Acme,
    Blnk,
    Crpt,
    Dyno,
    Echo,
    Flux,
    Grid,
    Helx,
    Ionx,
    Jolt,
    Krnl,
    Luma,
}

impl Ticker {
    pub fn name(self) -> &'static str {
        self.into()
    }
}

pub struct Stock {
    pub ticker: Ticker,
    pub country: Country,
    pub volatility: f32,
    pub dividend_yield: f32,
    pub history: VecDeque<f32>,
}

impl Stock {
    pub fn price(&self) -> f32 {
        self.history.back().copied().unwrap_or(STOCK_MIN_PRICE)
    }

    pub fn div_per_share(&self) -> f32 {
        self.price() * self.dividend_yield
    }

    pub fn weekly_change_pct(&self) -> f32 {
        let mut newest_first = self.history.iter().rev();
        match (newest_first.next(), newest_first.next()) {
            (Some(now), Some(before)) => (now / before - 1.0) * 100.0,
            _ => 0.0,
        }
    }

    pub fn adj_price(&mut self, boost: Boost) {
        if let Some(price) = self.history.back_mut() {
            *price = match boost {
                Boost::Add(amount) => *price + amount as f32,
                Boost::Mul(factor) => *price * factor as f32,
            }
            .max(STOCK_MIN_PRICE);
        }
    }

    pub(super) fn record(&mut self, price: f32) {
        self.history.push_back(price);
        while self.history.len() > MARKET_HISTORY_WEEKS {
            self.history.pop_front();
        }
    }
}

impl Country {
    pub fn name(self) -> &'static str {
        self.into()
    }
}

use Country::*;
use Ticker::*;

/// Ticker, country, start price, volatility, dividend yield.
pub(super) const STOCKS: [(Ticker, Country, f32, f32, f32); 12] = [
    (Acme, UnitedStates, 100.0, 0.03, 0.004),
    (Blnk, Germany, 12.0, 0.08, 0.0),
    (Crpt, France, 250.0, 0.10, 0.0),
    (Dyno, Japan, 45.0, 0.04, 0.002),
    (Echo, UnitedKingdom, 80.0, 0.02, 0.006),
    (Flux, UnitedStates, 30.0, 0.06, 0.0),
    (Grid, Germany, 150.0, 0.03, 0.005),
    (Helx, China, 60.0, 0.05, 0.002),
    (Ionx, UnitedStates, 20.0, 0.09, 0.0),
    (Jolt, France, 95.0, 0.04, 0.003),
    (Krnl, Japan, 180.0, 0.02, 0.007),
    (Luma, UnitedKingdom, 8.0, 0.12, 0.0),
];
