mod game;
mod game_over;
mod jokers;
mod portfolio;
mod style;
mod ticker;

use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;

use crate::general::AppState;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        // Egui is immediate mode: the whole UI is redrawn every frame.
        app.add_systems(
            EguiPrimaryContextPass,
            (
                game::game_ui,
                jokers::jokers_ui,
                ticker::ticker_ui,
                portfolio::portfolio_ui,
                game_over::game_over_ui.run_if(in_state(AppState::GameOver)),
            )
                .chain(),
        );
    }
}
