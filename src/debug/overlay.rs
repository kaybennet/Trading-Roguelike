use bevy::diagnostic::{
    DiagnosticPath, DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::window::PrimaryWindow;
use bevy_egui::{
    EguiContexts, EguiPrimaryContextPass,
    egui::{self, Color32, Id, Order},
};

use super::vsync::vsync_enabled;
use crate::consts::{DEBUG_BG_COLOR, DEBUG_FONT_SIZE, DEBUG_MARGIN, DEBUG_UPDATE_INTERVAL};
use crate::input::Toggle;

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin::default(),
            EntityCountDiagnosticsPlugin::default(),
        ))
        .init_resource::<DebugOverlay>()
        .add_systems(
            Update,
            (
                toggle_overlay,
                update_overlay.run_if(on_timer(DEBUG_UPDATE_INTERVAL)),
            ),
        )
        .add_systems(EguiPrimaryContextPass, draw_overlay);
    }
}

#[derive(Resource, Default)]
struct DebugOverlay {
    visible: bool,
    text: String,
}

fn toggle_overlay(mut messages: MessageReader<Toggle>, mut overlay: ResMut<DebugOverlay>) {
    for _ in messages.read().filter(|&&t| t == Toggle::DebugOverlay) {
        overlay.visible = !overlay.visible;
    }
}

fn update_overlay(
    diagnostics: Res<DiagnosticsStore>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut overlay: ResMut<DebugOverlay>,
) {
    if !overlay.visible {
        return;
    }
    let get = |path: &DiagnosticPath| {
        diagnostics
            .get(path)
            .and_then(|d| d.smoothed())
            .unwrap_or(0.0)
    };
    overlay.text = format!(
        "FPS: {:.0}\nFrame: {:.2} ms\nEntities: {:.0}\nVSync: {}",
        get(&FrameTimeDiagnosticsPlugin::FPS),
        get(&FrameTimeDiagnosticsPlugin::FRAME_TIME),
        get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT),
        if vsync_enabled(window.present_mode) {
            "on"
        } else {
            "off"
        },
    );
}

fn draw_overlay(mut contexts: EguiContexts, overlay: Res<DebugOverlay>) -> Result {
    if !overlay.visible {
        return Ok(());
    }
    let ctx = contexts.ctx_mut()?;
    let bg = DEBUG_BG_COLOR.to_srgba();
    let fill = Color32::from_rgb(
        (bg.red * 255.0) as u8,
        (bg.green * 255.0) as u8,
        (bg.blue * 255.0) as u8,
    );
    egui::Area::new(Id::new("debug_overlay"))
        .order(Order::Debug)
        .fixed_pos(egui::pos2(DEBUG_MARGIN, DEBUG_MARGIN))
        .interactable(false)
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(fill)
                .inner_margin(4.0)
                .show(ui, |ui| {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(&overlay.text)
                                .size(DEBUG_FONT_SIZE)
                                .color(Color32::WHITE),
                        )
                        .selectable(false),
                    );
                });
        });
    Ok(())
}
