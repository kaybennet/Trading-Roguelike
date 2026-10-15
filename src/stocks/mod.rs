mod market;
mod offer;
mod stocks;

use bevy::prelude::*;

use crate::general::{AppState, GameEvent, GlobalRng, turns_ended};

pub use market::Market;
pub use offer::WeeklyOffer;
pub use stocks::{Stock, Ticker};

pub struct StocksPlugin;

impl Plugin for StocksPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Market>()
            .add_plugins(offer::OfferPlugin)
            // A new run starts with a fresh market.
            .add_systems(OnExit(AppState::GameOver), market::create_market)
            .add_systems(Startup, market::create_market)
            .add_systems(Update, evolve_prices);
    }
}

/// Prices only move when a turn (week) ends.
fn evolve_prices(
    mut events: MessageReader<GameEvent>,
    mut rng: ResMut<GlobalRng>,
    mut market: ResMut<Market>,
) {
    if turns_ended(&mut events) == 0 {
        return;
    }
    market.advance_week(&mut rng);
}
