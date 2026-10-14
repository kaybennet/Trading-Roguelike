mod chart;
mod flash;
mod game;
mod game_over;
mod joker_shop;
mod jokers;
mod market;
mod portfolio;
mod style;
mod ticker;

use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;

use crate::general::AppState;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<flash::Flashes>()
            .add_systems(Update, flash::update_flashes)
            // Egui is immediate mode: the whole UI is redrawn every frame.
            .add_systems(
                EguiPrimaryContextPass,
                (
                    style::apply_theme,
                    game::game_ui,
                    jokers::jokers_ui,
                    ticker::ticker_ui,
                    portfolio::portfolio_ui,
                    market::market_ui,
                    joker_shop::joker_shop_ui.run_if(in_state(AppState::JokerShop)),
                    game_over::game_over_ui.run_if(in_state(AppState::GameOver)),
                )
                    .chain(),
            );
    }
}
