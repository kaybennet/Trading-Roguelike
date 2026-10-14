use strum::{EnumIter, IntoStaticStr};

use crate::general::{GameState, Portfolio};
use crate::rarity::Rarity;

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

    pub fn rarity(self) -> Rarity {
        match self {
            Self::Wallet => Rarity::Common,
            Self::Compound | Self::Savings => Rarity::Uncommon,
            Self::Bonds => Rarity::Rare,
            Self::Leverage => Rarity::Epic,
        }
    }

    pub fn on_buy(self, portfolio: &mut Portfolio, state: &mut GameState) {
        match self {
            Self::Wallet => portfolio.max_size += 1,
            _ => state.interest += 0.02,
        }
    }
}
