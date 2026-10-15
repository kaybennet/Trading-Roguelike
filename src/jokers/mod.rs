mod id;
mod listeners;

use bevy::prelude::*;

pub use id::Joker;

pub struct JokersPlugin;

impl Plugin for JokersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(listeners::ListenersPlugin);
    }
}
