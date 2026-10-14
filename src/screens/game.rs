use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Align2, Color32, FontId, Id, Pos2},
};

use super::market;
use super::style::{
    backdrop, button, c32, card, draw_background, lines_height, muted, place, rect_at,
};
use crate::colors::{ACCENT_COLOR, PANEL_COLOR, SCORE_BOX_COLOR};
use crate::consts::{
    JOKER_PANEL_WIDTH_PCT, PANEL_GAP, SCORE_FONT_SIZE, SCORE_PANEL_WIDTH_PCT, TOP_BAR_FONT_SIZE,
    TOP_BAR_HEIGHT_PCT,
};
use crate::general::{Date, GameState, Portfolio, Sequence, TurnEnded};

const LABEL_FONT_SIZE: f32 = 14.0;
const CONTINUE_HEIGHT: f32 = 48.0;
/// How long the displayed cash takes to catch up with the real value.
const CASH_COUNT_SECS: f32 = 0.35;

/// Left stats box, top bar, and Continue button.
pub fn game_ui(
    mut contexts: EguiContexts,
    state: Res<GameState>,
    portfolio: Res<Portfolio>,
    mut turns: MessageWriter<TurnEnded>,
    sequence: Res<Sequence>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let screen = ctx.content_rect();
    let (w, h) = (screen.width(), screen.height());
    let left_w = w * SCORE_PANEL_WIDTH_PCT / 100.0;
    let bg = backdrop(ctx);
    draw_background(&bg, screen);

    // Left box: stats from the top, Continue at the bottom.
    let side = rect_at(
        PANEL_GAP,
        PANEL_GAP,
        left_w - PANEL_GAP,
        h - 2.0 * PANEL_GAP,
    );
    card(&bg, side, SCORE_BOX_COLOR, 0.0, 0.0);
    let stats = [
        (
            "WEEK",
            format!("{}/{}", state.date.week + 1, Date::WEEKS_PER_MONTH),
        ),
        ("MONTH", state.date.month_name().to_string()),
        ("YEAR", (state.date.year + 1).to_string()),
        ("INTEREST", format!("{:.0}%", state.interest * 100.0)),
    ];
    let mut y = side.top() + PANEL_GAP;
    for (label, value) in stats {
        let x = side.left() + PANEL_GAP;
        let drawn = bg.text(
            Pos2::new(x, y),
            Align2::LEFT_TOP,
            label,
            FontId::proportional(LABEL_FONT_SIZE),
            muted(),
        );
        let drawn = bg.text(
            Pos2::new(x, drawn.bottom() + 2.0),
            Align2::LEFT_TOP,
            value,
            FontId::proportional(SCORE_FONT_SIZE + 4.0),
            Color32::WHITE,
        );
        y = drawn.bottom() + PANEL_GAP;
    }

    // Top bar: between the left box and the joker column, values from the left.
    let bar_x = left_w + PANEL_GAP;
    let bar_w = w - w * JOKER_PANEL_WIDTH_PCT / 100.0 - bar_x;
    let bar_h = h * TOP_BAR_HEIGHT_PCT / 100.0;
    let bar = rect_at(bar_x, PANEL_GAP, bar_w, bar_h);
    card(&bg, bar, PANEL_COLOR, 0.0, 0.0);
    // The cash counts up to its new value instead of jumping.
    let cash = portfolio.cash;
    let animated = ctx.animate_value_with_time(Id::new("cash_shown"), cash as f32, CASH_COUNT_SECS);
    let shown = if animated == cash as f32 {
        cash
    } else {
        animated as f64
    };
    let values = [
        ("CASH", format!("€{shown:.2}"), c32(ACCENT_COLOR)),
        (
            "INVESTED",
            format!("€{:.2}", state.invested_euro),
            Color32::WHITE,
        ),
        (
            "MONTHLY DEBT",
            format!("€{:.2}", state.monthly_debt_euro),
            Color32::WHITE,
        ),
    ];
    let block_h = lines_height(1, LABEL_FONT_SIZE) + lines_height(1, TOP_BAR_FONT_SIZE);
    let top = bar.center().y - block_h / 2.0;
    let mut x = bar.left() + PANEL_GAP * 1.5;
    for (label, value, color) in values {
        let label = bg.text(
            Pos2::new(x, top),
            Align2::LEFT_TOP,
            label,
            FontId::proportional(LABEL_FONT_SIZE),
            muted(),
        );
        let value = bg.text(
            Pos2::new(x, label.bottom() + 2.0),
            Align2::LEFT_TOP,
            value,
            FontId::proportional(TOP_BAR_FONT_SIZE),
            color,
        );
        x = value.right().max(label.right()) + PANEL_GAP * 3.0;
    }

    // Bottom of the left box: end the turn (one week).
    let continue_size = egui::vec2(side.width() - 2.0 * PANEL_GAP, CONTINUE_HEIGHT);
    let continue_rect = rect_at(
        side.left() + PANEL_GAP,
        side.bottom() - PANEL_GAP - CONTINUE_HEIGHT,
        continue_size.x,
        continue_size.y,
    );
    // Above it: open the full-screen market with every stock's chart.
    let market_rect = continue_rect.translate(egui::vec2(0.0, -CONTINUE_HEIGHT - PANEL_GAP));
    place(ctx, "market_button", market_rect, |ui| {
        if button(ui, "Market", TOP_BAR_FONT_SIZE, continue_size).clicked() {
            ui.data_mut(|d| d.insert_temp(market::open_id(), true));
        }
    });
    place(ctx, "continue", continue_rect, |ui| {
        // Locked until the end-of-turn triggers have all played out.
        ui.add_enabled_ui(!sequence.is_busy(), |ui| {
            if button(ui, "Continue", TOP_BAR_FONT_SIZE, continue_size).clicked() {
                turns.write(TurnEnded);
            }
        });
    });
    Ok(())
}
