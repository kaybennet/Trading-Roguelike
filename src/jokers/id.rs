use strum::{EnumIter, IntoStaticStr};

use crate::general::{GameState, Portfolio};

#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumIter, IntoStaticStr)]
pub enum JokerId {
    Bull,
    Bear,
    Hedge,
    Dividend,
    #[strum(serialize = "Triple Dividend")]
    TripleDividend,
    Insider,
    #[strum(serialize = "Day Trader")]
    DayTrader,
    Whale,
    Penny,
}

impl JokerId {
    pub fn name(self) -> &'static str {
        self.into()
    }

    pub fn price(self) -> f64 {
        match self {
            Self::Bull | Self::Bear => 30.0,
            Self::Hedge => 40.0,
            Self::Dividend => 25.0,
            Self::TripleDividend => 55.0,
            Self::Insider => 60.0,
            Self::DayTrader => 35.0,
            Self::Whale => 80.0,
            Self::Penny => 15.0,
        }
    }

    pub fn draw_weight(self) -> u32 {
        match self {
            Self::Penny | Self::Dividend => 10,
            Self::Bull | Self::Bear | Self::DayTrader => 8,
            Self::Hedge => 5,
            Self::Insider | Self::TripleDividend => 3,
            Self::Whale => 1,
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::TripleDividend => "Dividends trigger 3x",
            _ => "+€1 on buy",
        }
    }

    /// How many extra times this joker triggers a dividend it hears about.
    pub fn dividend_retriggers(self) -> usize {
        match self {
            Self::TripleDividend => 2,
            _ => 0,
        }
    }

    pub fn on_buy(self, portfolio: &mut Portfolio) {
        match self {
            _ => portfolio.cash += 1.,
        }
    }

    /// What the joker does when a turn ends, or `None` if it does nothing then.
    pub fn turn_effect(self) -> Option<fn(&mut Portfolio, &mut GameState)> {
        match self {
            Self::TripleDividend => None,
            _ => Some(|portfolio, _| portfolio.cash += 50.),
        }
    }
}
