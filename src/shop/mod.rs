use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::consts::{JOKER_COUNT, SHOP_OFFER_SIZE};
use crate::general::{AppState, GameState, GlobalRng, Portfolio};
use crate::jokers::Joker;
use crate::modifiers::ModifierId;
use crate::rarity::Rarity;

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShopOffer>()
            .add_systems(OnEnter(AppState::JokerShop), roll_offer);
    }
}

/// Something the shop sells. Jokers and modifiers are drawn from one pool.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Card {
    Joker(Joker),
    Modifier(ModifierId),
}

impl Card {
    pub fn name(self) -> &'static str {
        match self {
            Self::Joker(joker) => joker.name(),
            Self::Modifier(modifier) => modifier.name(),
        }
    }

    pub fn price(self) -> f64 {
        match self {
            Self::Joker(joker) => joker.price(),
            Self::Modifier(modifier) => modifier.price(),
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Joker(joker) => joker.description(),
            Self::Modifier(modifier) => modifier.description(),
        }
    }

    pub fn rarity(self) -> Rarity {
        match self {
            Self::Joker(joker) => joker.rarity(),
            Self::Modifier(modifier) => modifier.rarity(),
        }
    }

    pub fn can_buy(self, portfolio: &Portfolio) -> bool {
        let room = match self {
            Self::Joker(_) => portfolio.jokers.len() < JOKER_COUNT as usize,
            Self::Modifier(_) => true,
        };
        room && portfolio.cash >= self.price()
    }
}

/// The cards currently on sale in the shop.
#[derive(Resource, Default)]
pub struct ShopOffer(pub Vec<Card>);

impl ShopOffer {
    /// Buys the card at `index`: pays for it, keeps it and runs its on-buy effect.
    pub fn buy(&mut self, index: usize, portfolio: &mut Portfolio, state: &mut GameState) -> bool {
        let Some(&card) = self.0.get(index) else {
            return false;
        };
        if !card.can_buy(portfolio) {
            return false;
        }
        self.0.remove(index);
        portfolio.cash -= card.price();
        match card {
            Card::Joker(joker) => {
                portfolio.jokers.push(joker);
                joker.on_buy(portfolio);
            }
            Card::Modifier(modifier) => {
                state.modifiers.push(modifier);
                modifier.on_buy(portfolio, state);
            }
        }
        true
    }
}

fn roll_offer(mut offer: ResMut<ShopOffer>, mut rng: ResMut<GlobalRng>) {
    let pool = Joker::iter()
        .map(Card::Joker)
        .chain(ModifierId::iter().map(Card::Modifier))
        .collect();
    offer.0 = rng.draw_weighted(pool, SHOP_OFFER_SIZE, |card| card.rarity().draw_weight());
}
