use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Color32, Id, Order, Sense},
};

use super::style::{c32, centered_lines, lines_height};
use crate::consts::{GAME_OVER_BG_COLOR, GAME_OVER_FONT_SIZE};
use crate::general::{AppState, GameState};

/// Full-screen cover shown on game over. It sits above the game, so the
/// game can't be played underneath; a click restarts the run.
pub fn game_over_ui(
    mut contexts: EguiContexts,
    state: Res<GameState>,
    mut next_state: ResMut<NextState<AppState>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let screen = ctx.content_rect();
    egui::Area::new(Id::new("game_over"))
        .order(Order::Foreground)
        .fixed_pos(screen.min)
        .show(ctx, |ui| {
            let response = ui.allocate_rect(screen, Sense::click());
            let painter = ui.painter();
            painter.rect_filled(screen, 0.0, c32(GAME_OVER_BG_COLOR));
            let lines = [
                "GAME OVER".to_string(),
                String::new(),
                format!(
                    "You could not pay the €{:.2} debt in {}.",
                    state.monthly_debt_euro,
                    state.date.month_name()
                ),
                String::new(),
                "Click to restart".to_string(),
            ];
            centered_lines(
                painter,
                screen.center().x,
                screen.center().y - lines_height(lines.len(), GAME_OVER_FONT_SIZE) / 2.0,
                &lines,
                GAME_OVER_FONT_SIZE,
                Color32::WHITE,
            );
            if response.clicked() {
                next_state.set(AppState::Playing);
            }
        });
    Ok(())
}
