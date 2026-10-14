use bevy::color::Mix;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use super::flash::Flashes;
use super::style::{backdrop, button, c32, centered_lines, lines_height, place, rect_at};
use crate::consts::{
    PANEL_COLOR, PANEL_GAP, SCORE_PANEL_WIDTH_PCT, SELL_FRACTIONS, SQUARE_BUTTON_FONT_SIZE,
    SQUARE_HEIGHT_PCT, STOCK_DOWN_COLOR, STOCK_FONT_SIZE, STOCK_UP_COLOR, TRIGGER_FLASH_COLOR,
};
use crate::general::Portfolio;
use crate::stocks::Market;

const BUTTON_GAP: f32 = 4.0;

/// Bottom row of squares: the held stocks, each with its sell buttons.
pub fn portfolio_ui(
    mut contexts: EguiContexts,
    mut portfolio: ResMut<Portfolio>,
    market: Res<Market>,
    flashes: Res<Flashes>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let screen = ctx.content_rect();
    let (w, h) = (screen.width(), screen.height());
    let side = h * SQUARE_HEIGHT_PCT / 100.0;
    let left = w * SCORE_PANEL_WIDTH_PCT / 100.0 + PANEL_GAP;
    let count = portfolio.max_size as f32;
    // Squares are spread evenly over the width right of the left box.
    let spacing = ((w - PANEL_GAP - left - count * side) / (count + 1.0)).max(BUTTON_GAP);
    let pad = PANEL_GAP / 2.0;
    let button_h = SQUARE_BUTTON_FONT_SIZE + 10.0;
    let button_w = (side - 2.0 * pad - BUTTON_GAP * (SELL_FRACTIONS.len() as f32 - 1.0))
        / SELL_FRACTIONS.len() as f32;
    let bg = backdrop(ctx);

    for slot in 0..portfolio.max_size {
        let x = left + spacing * (slot as f32 + 1.0) + side * slot as f32;
        let square = rect_at(x, h - PANEL_GAP - side, side, side);
        let color = PANEL_COLOR.mix(&TRIGGER_FLASH_COLOR, flashes.holding(slot));
        bg.rect_filled(square, 0.0, c32(color));

        let held = portfolio
            .slots
            .get(slot)
            .cloned()
            .flatten()
            .and_then(|holding| {
                let stock = market.get(&holding.symbol)?;
                Some((holding, stock))
            });
        match &held {
            Some((holding, stock)) => {
                let value = holding.shares * stock.price() as f64;
                let gain = (stock.price() as f64 / holding.avg_cost - 1.0) * 100.0;
                let color = c32(if gain >= 0.0 {
                    STOCK_UP_COLOR
                } else {
                    STOCK_DOWN_COLOR
                });
                let lines = [
                    format!("{} x{:.2}", holding.symbol, holding.shares),
                    format!("€{value:.2}"),
                    format!("{gain:+.1}%"),
                ];
                centered_lines(
                    &bg,
                    square.center().x,
                    square.top() + pad,
                    &lines,
                    STOCK_FONT_SIZE,
                    color,
                );
            }
            None => {
                let text_area = side - button_h - 2.0 * pad;
                let top = square.top() + pad + (text_area - lines_height(1, STOCK_FONT_SIZE)) / 2.0;
                centered_lines(
                    &bg,
                    square.center().x,
                    top,
                    &["Empty".to_string()],
                    STOCK_FONT_SIZE,
                    egui::Color32::WHITE,
                );
            }
        }

        let buttons = egui::Rect::from_min_size(
            egui::pos2(square.left() + pad, square.bottom() - pad - button_h),
            egui::vec2(side - 2.0 * pad, button_h),
        );
        place(ctx, ("sell", slot), buttons, |ui| {
            ui.spacing_mut().item_spacing.x = BUTTON_GAP;
            ui.horizontal(|ui| {
                for (label, fraction) in SELL_FRACTIONS {
                    let size = egui::vec2(button_w, button_h);
                    if button(ui, label, SQUARE_BUTTON_FONT_SIZE, size).clicked()
                        && let Some((_, stock)) = &held
                    {
                        portfolio.sell(stock, fraction);
                    }
                }
            });
        });
    }
    Ok(())
}
