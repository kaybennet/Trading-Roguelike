use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow};

use crate::input::Toggle;

pub struct VsyncPlugin;

impl Plugin for VsyncPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_vsync);
    }
}

/// Whether the present mode waits for the display refresh.
pub fn vsync_enabled(mode: PresentMode) -> bool {
    matches!(mode, PresentMode::AutoVsync | PresentMode::Fifo | PresentMode::FifoRelaxed)
}

fn toggle_vsync(
    mut messages: MessageReader<Toggle>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
) {
    for _ in messages.read().filter(|t| **t == Toggle::Vsync) {
        window.present_mode = if vsync_enabled(window.present_mode) {
            PresentMode::AutoNoVsync
        } else {
            PresentMode::AutoVsync
        };
    }
}
