use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Align2, Color32, FontId, Pos2},
};

use super::style::{backdrop, button, c32, centered_lines, lines_height, place, rect_at};
use crate::consts::{
    CIRCLE_FONT_SIZE, CIRCLE_SIZE, JOKER_PANEL_WIDTH_PCT, PANEL_COLOR, PANEL_GAP, SCORE_BOX_COLOR,
    SCORE_FONT_SIZE, SCORE_PANEL_WIDTH_PCT, TOP_BAR_FONT_SIZE, TOP_BAR_HEIGHT_PCT,
};
use crate::general::{Date, GameState, Portfolio, TurnEnded};

const CONTINUE_SIZE: egui::Vec2 = egui::vec2(180.0, 48.0);

/// Left stats box, top bar, Continue button and interest circle.
pub fn game_ui(
    mut contexts: EguiContexts,
    state: Res<GameState>,
    portfolio: Res<Portfolio>,
    mut turns: MessageWriter<TurnEnded>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let screen = ctx.content_rect();
    let (w, h) = (screen.width(), screen.height());
    let left_w = w * SCORE_PANEL_WIDTH_PCT / 100.0;
    let bg = backdrop(ctx);

    // Left box: full height, stats from the top left.
    bg.rect_filled(rect_at(0.0, 0.0, left_w, h), 0.0, c32(SCORE_BOX_COLOR));
    let stats = [
        format!("Week: {}/{}", state.date.week + 1, Date::WEEKS_PER_MONTH),
        format!("Month: {}", state.date.month_name()),
        format!("Year: {}", state.date.year + 1),
    ];
    let mut y = PANEL_GAP;
    for line in stats {
        let drawn = bg.text(
            Pos2::new(PANEL_GAP, y),
            Align2::LEFT_TOP,
            line,
            FontId::proportional(SCORE_FONT_SIZE),
            Color32::WHITE,
        );
        y = drawn.bottom() + PANEL_GAP;
    }

    // Top bar: between the left box and the joker column, values from the left.
    let bar_x = left_w + PANEL_GAP;
    let bar_w = w - w * JOKER_PANEL_WIDTH_PCT / 100.0 - bar_x;
    let bar_h = h * TOP_BAR_HEIGHT_PCT / 100.0;
    bg.rect_filled(
        rect_at(bar_x, PANEL_GAP, bar_w, bar_h),
        0.0,
        c32(PANEL_COLOR),
    );
    let values = [
        format!("Cash: €{:.2}", portfolio.cash),
        format!("Invested: €{:.2}", state.invested_euro),
        format!("Monthly debt: €{:.2}", state.monthly_debt_euro),
    ];
    let mut x = bar_x + PANEL_GAP;
    for value in values {
        let drawn = bg.text(
            Pos2::new(x, PANEL_GAP * 2.0),
            Align2::LEFT_TOP,
            value,
            FontId::proportional(TOP_BAR_FONT_SIZE),
            Color32::WHITE,
        );
        x = drawn.right() + PANEL_GAP * 3.0;
    }

    // Top right: interest circle.
    let radius = CIRCLE_SIZE / 2.0;
    let center = Pos2::new(w - PANEL_GAP - radius, PANEL_GAP + radius);
    bg.circle_filled(center, radius, c32(PANEL_COLOR));
    let label = [
        format!("{:.0}%", state.interest * 100.0),
        "interest".to_string(),
    ];
    centered_lines(
        &bg,
        center.x,
        center.y - lines_height(label.len(), CIRCLE_FONT_SIZE) / 2.0,
        &label,
        CIRCLE_FONT_SIZE,
        Color32::WHITE,
    );

    // Bottom left: end the turn (one week).
    let continue_rect = rect_at(
        PANEL_GAP,
        h - PANEL_GAP - CONTINUE_SIZE.y,
        CONTINUE_SIZE.x,
        CONTINUE_SIZE.y,
    );
    place(ctx, "continue", continue_rect, |ui| {
        if button(ui, "Continue", TOP_BAR_FONT_SIZE, CONTINUE_SIZE).clicked() {
            turns.write(TurnEnded);
        }
    });
    Ok(())
}
