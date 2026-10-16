mod joker;
mod listeners;

use bevy::prelude::*;

pub use joker::{Joker, JokerKind};

pub struct JokersPlugin;

impl Plugin for JokersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(listeners::ListenersPlugin);
    }
}
