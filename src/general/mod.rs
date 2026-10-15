mod app_state;
mod game_events;
mod game_state;
mod portfolio;
mod turn;

use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

use crate::consts::SEED;

pub use app_state::AppState;
pub use game_events::{Boost, CashGained, GameEvent, SharesGained, Sold, Triggered, turns_ended};
pub use game_state::{Date, GameState};
pub use portfolio::Portfolio;
pub use turn::Sequence;

/// The one random number generator, seeded with `SEED`.
#[derive(Resource, Deref, DerefMut)]
pub struct GlobalRng(StdRng);

impl GlobalRng {
    /// Draws `count` distinct items from `pool`, each pick chosen by `weight`.
    pub fn draw_weighted<T>(
        &mut self,
        mut pool: Vec<T>,
        count: usize,
        weight: impl Fn(&T) -> u32,
    ) -> Vec<T> {
        let mut drawn = Vec::new();
        while drawn.len() < count && !pool.is_empty() {
            let total: u32 = pool.iter().map(&weight).sum();
            let mut roll = self.random_range(0..total);
            let index = pool
                .iter()
                .position(|item| {
                    let w = weight(item);
                    let hit = roll < w;
                    roll = roll.saturating_sub(w);
                    hit
                })
                .unwrap_or(0);
            drawn.push(pool.remove(index));
        }
        drawn
    }
}

pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalRng(StdRng::seed_from_u64(SEED)))
            .add_plugins((
                game_events::GameEventsPlugin,
                app_state::AppStatePlugin,
                game_state::GameStatePlugin,
                turn::TurnPlugin,
                portfolio::PortfolioPlugin,
            ));
    }
}
