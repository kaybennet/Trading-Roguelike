mod consts;
mod debug;
mod general;
mod input;
mod jokers;
mod modifiers;
mod screens;
mod shop;
mod stocks;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin::default(),
            input::InputPlugin,
            debug::DebugPlugin,
            general::GeneralPlugin,
            screens::ScreensPlugin,
            stocks::StocksPlugin,
            jokers::JokersPlugin,
            shop::ShopPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
