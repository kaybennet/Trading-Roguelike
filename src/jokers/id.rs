use strum::{EnumIter, IntoStaticStr};

use crate::general::{GameState, GlobalRng, Portfolio};
use crate::rarity::Rarity;

#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumIter, IntoStaticStr)]
pub enum Joker {
    #[strum(serialize = "Double Dividend")]
    DoubleDividend,
    #[strum(serialize = "Triple Dividend")]
    TripleDividend,
    #[strum(serialize = "Quadruple Dividend")]
    QuadrupleDividend,
    #[strum(serialize = "Increased Dividend")]
    IncreasedDividend,
    #[strum(serialize = "Magic Sell")]
    MagicSell,
    #[strum(serialize = "Steady Income")]
    SteadyIncome,
    #[strum(serialize = "Autonomic Gain")]
    AutonomicGain,
}

use Joker::*;
use Rarity::*;
impl Joker {
    pub fn name(self) -> &'static str {
        self.into()
    }

    pub fn price(self) -> f64 {
        match self {
            DoubleDividend => 15.0,
            TripleDividend => 25.0,
            QuadrupleDividend => 40.0,
            MagicSell => 20.0,
            SteadyIncome => 10.0,
            AutonomicGain => 20.,
            IncreasedDividend => 10.,
        }
    }

    pub fn rarity(self) -> Rarity {
        match self {
            DoubleDividend => Rare,
            TripleDividend => Legendary,
            QuadrupleDividend => Mythic,
            MagicSell => Uncommon,
            SteadyIncome => Common,
            AutonomicGain => Rare,
            IncreasedDividend => Common,
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            DoubleDividend => "Dividends trigger x2 (stacks by multiplying)",
            TripleDividend => "Dividends trigger x3 (stacks by multiplying)",
            QuadrupleDividend => "Dividends trigger x4 (stacks by multiplying)",
            MagicSell => "Selling a share raises that stock's price by 1%",
            SteadyIncome => "Gain €5 per week",
            AutonomicGain => "Every week gain 1 share of a random stock in your portfolio",
            IncreasedDividend => "Increases Dividends in your Portfolio by €1",
        }
    }

    pub fn on_buy(self, portfolio: &mut Portfolio) {
        match self {
            _ => {}
        }
    }

    pub fn turn_effect(self) -> Option<fn(&mut Portfolio, &mut GameState, &mut GlobalRng)> {
        match self {
            SteadyIncome => Some(|p, _, _| p.cash += 5.),
            AutonomicGain => Some(|p, _, rng| {
                if let Some(holding) = p.rnd_holding(rng) {
                    holding.add_free_share();
                }
            }),
            _ => None,
        }
    }
}
