mod app_state;
mod game_state;
mod portfolio;
mod turn;

use bevy::prelude::*;
use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::consts::SEED;

pub use app_state::AppState;
pub use game_state::{Date, GameState};
pub use portfolio::Portfolio;
pub use turn::TurnEnded;

/// The one random number generator, seeded with `SEED`.
#[derive(Resource, Deref, DerefMut)]
pub struct GlobalRng(StdRng);

pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalRng(StdRng::seed_from_u64(SEED)))
            .add_plugins((
                app_state::AppStatePlugin,
                game_state::GameStatePlugin,
                turn::TurnPlugin,
                portfolio::PortfolioPlugin,
            ));
    }
}
