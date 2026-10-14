use strum::{EnumIter, IntoStaticStr};

use crate::general::{GameState, Portfolio};

#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumIter, IntoStaticStr)]
pub enum ModifierId {
    Compound,
    Bonds,
    Leverage,
    Savings,
    Wallet,
}

impl ModifierId {
    pub fn name(self) -> &'static str {
        self.into()
    }

    pub fn price(self) -> f64 {
        match self {
            Self::Wallet => 10.,
            Self::Compound | Self::Savings => 50.0,
            Self::Bonds | Self::Leverage => 70.0,
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Wallet => "increase stock amount you can hold",
            _ => "+2% interest",
        }
    }

    pub fn draw_weight(self) -> u32 {
        match self {
            Self::Wallet => 10,
            Self::Compound | Self::Savings => 3,
            Self::Bonds | Self::Leverage => 2,
        }
    }

    pub fn on_buy(self, portfolio: &mut Portfolio, state: &mut GameState) {
        match self {
            Self::Wallet => portfolio.max_size += 1,
            _ => state.interest += 0.02,
        }
    }
}
