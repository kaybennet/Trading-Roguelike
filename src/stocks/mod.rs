mod market;
mod offer;
mod stocks;

use bevy::prelude::*;

use crate::general::{AppState, GlobalRng};

pub use market::Market;
pub use offer::WeeklyOffer;
pub use stocks::{Stock, Ticker};

pub struct StocksPlugin;

impl Plugin for StocksPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Market>()
            .add_plugins(offer::OfferPlugin)
            // A new run starts with a fresh market.
            .add_systems(OnExit(AppState::GameOver), Market::create)
            .add_systems(Startup, Market::create)
            .add_systems(OnEnter(AppState::EndOfWeek), evolve_prices);
    }
}

/// Prices only move when a turn (week) ends.
fn evolve_prices(mut rng: ResMut<GlobalRng>, mut market: ResMut<Market>) {
    market.advance_week(&mut rng);
}
