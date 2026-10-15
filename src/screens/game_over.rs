use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Color32, Id, Order, Sense},
};

use super::style::{c32, centered_lines, lines_height};
use crate::colors::{GAME_OVER_BG_COLOR, STOCK_DOWN_COLOR};
use crate::consts::GAME_OVER_FONT_SIZE;
use crate::general::{AppState, GameState};

const FADE_SECS: f32 = 0.5;

/// Full-screen cover shown on game over. It sits above the game, so the
/// game can't be played underneath; a click restarts the run.
pub fn game_over_ui(
    mut contexts: EguiContexts,
    state: Res<GameState>,
    mut next_state: ResMut<NextState<AppState>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let screen = ctx.content_rect();
    let fade_id = Id::new("game_over_fade");
    let fade = ctx.animate_bool_with_time(fade_id, true, FADE_SECS);
    egui::Area::new(Id::new("game_over"))
        .order(Order::Foreground)
        .fixed_pos(screen.min)
        .show(ctx, |ui| {
            let response = ui.allocate_rect(screen, Sense::click());
            let painter = ui.painter();
            painter.rect_filled(screen, 0.0, c32(GAME_OVER_BG_COLOR).gamma_multiply(fade));
            let title_size = GAME_OVER_FONT_SIZE * 2.0;
            let lines = [
                format!(
                    "You could not pay the €{:.2} debt in {}.",
                    state.monthly_debt_euro,
                    state.date.month_name()
                ),
                String::new(),
                "Click to restart".to_string(),
            ];
            let block_h =
                lines_height(1, title_size) + lines_height(lines.len(), GAME_OVER_FONT_SIZE);
            let top = screen.center().y - block_h / 2.0;
            let bottom = centered_lines(
                painter,
                screen.center().x,
                top,
                &["GAME OVER".to_string()],
                title_size,
                c32(STOCK_DOWN_COLOR).gamma_multiply(fade),
            );
            centered_lines(
                painter,
                screen.center().x,
                bottom,
                &lines,
                GAME_OVER_FONT_SIZE,
                Color32::WHITE.gamma_multiply(fade),
            );
            if response.clicked() {
                next_state.set(AppState::DuringWeek);
                ui.ctx().animate_bool_with_time(fade_id, false, 0.0);
            }
        });
    Ok(())
}
