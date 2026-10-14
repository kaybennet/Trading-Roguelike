use bevy::color::Mix;
use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{Align2, Color32, FontId},
};

use super::flash::Flashes;
use super::style::{backdrop, c32, rect_at};
use crate::consts::{
    JOKER_COUNT, JOKER_FONT_SIZE, JOKER_HEIGHT_PCT, JOKER_PANEL_WIDTH_PCT, PANEL_COLOR, PANEL_GAP,
    TICKER_HEIGHT_PCT, TICKER_TOP_PCT, TRIGGER_FLASH_COLOR,
};
use crate::general::Portfolio;

/// Right side of the screen: a column of joker slots, vertically centred on the
/// same line as the weekly stock ticker.
pub fn jokers_ui(
    mut contexts: EguiContexts,
    portfolio: Res<Portfolio>,
    flashes: Res<Flashes>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let screen = ctx.content_rect();
    let (w, h) = (screen.width(), screen.height());
    let column_w = w * JOKER_PANEL_WIDTH_PCT / 100.0;
    let slot_h = h * JOKER_HEIGHT_PCT / 100.0;
    let count = JOKER_COUNT as f32;
    let total = count * slot_h + (count - 1.0) * PANEL_GAP;
    let top = h * (TICKER_TOP_PCT + TICKER_HEIGHT_PCT / 2.0) / 100.0 - total / 2.0;
    let bg = backdrop(ctx);

    for n in 1..=JOKER_COUNT {
        let y = top + (n - 1) as f32 * (slot_h + PANEL_GAP);
        let slot = rect_at(
            w - column_w + PANEL_GAP,
            y,
            column_w - 2.0 * PANEL_GAP,
            slot_h,
        );
        let color = PANEL_COLOR.mix(&TRIGGER_FLASH_COLOR, flashes.joker(n as usize - 1));
        bg.rect_filled(slot, 0.0, c32(color));
        bg.text(
            slot.center(),
            Align2::CENTER_CENTER,
            portfolio
                .jokers
                .get(n as usize - 1)
                .map_or_else(|| format!("Joker{n}"), |joker| joker.name().to_string()),
            FontId::proportional(JOKER_FONT_SIZE),
            Color32::WHITE,
        );
    }
    Ok(())
}
