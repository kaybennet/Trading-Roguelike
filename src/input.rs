use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Toggle>()
            .add_systems(PreUpdate, keyboard_input);
    }
}

/// Sent when the player asks to switch something on or off.
#[derive(Message, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Toggle {
    DebugOverlay,
    Vsync,
}

impl Toggle {
    const ALL: [Toggle; 2] = [Toggle::DebugOverlay, Toggle::Vsync];

    const fn key(self) -> KeyCode {
        use KeyCode::*;
        use Toggle::*;
        match self {
            DebugOverlay => F3,
            Vsync => KeyV,
        }
    }
}

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut toggles: MessageWriter<Toggle>) {
    for toggle in Toggle::ALL {
        if keys.just_pressed(toggle.key()) {
            toggles.write(toggle);
        }
    }
}
