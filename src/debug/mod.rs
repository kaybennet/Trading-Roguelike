mod overlay;
mod vsync;

use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((overlay::OverlayPlugin, vsync::VsyncPlugin));
    }
}
