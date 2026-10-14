use bevy::color::Mix;
use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{Align2, Color32, CornerRadius, CursorIcon, FontId, Id, PointerButton, Pos2, Sense},
};

use super::chart::{sparkline, trend_color};
use super::flash::{Anchor, Flashes};
use super::style::{
    backdrop, c32, card, centered_lines, held_on, hold_repeat, hover_amount, lines_height, muted,
    place, rect_at,
};
use crate::colors::{PANEL_COLOR, SCORE_BOX_COLOR, STOCK_DOWN_COLOR, STOCK_UP_COLOR, stock_color};
use crate::consts::{PANEL_GAP, SCORE_PANEL_WIDTH_PCT, SQUARE_HEIGHT_PCT, STOCK_FONT_SIZE};
use crate::general::{Portfolio, Sold};
use crate::stocks::Market;

const MIN_SPACING: f32 = 4.0;
const BAR_HEIGHT: f32 = 5.0;
const SYMBOL_SIZE: f32 = 20.0;
const VALUE_SIZE: f32 = 18.0;
const DETAIL_SIZE: f32 = 13.0;
const HINT_SIZE: f32 = 11.0;
/// How many of the latest weekly prices the square's sparkline shows.
const SPARKLINE_WEEKS: usize = 24;

/// Bottom row of squares: the held stocks. Hover one for its full price chart;
/// hold left click or right click to sell shares.
pub fn portfolio_ui(
    mut contexts: EguiContexts,
    mut portfolio: ResMut<Portfolio>,
    market: Res<Market>,
    flashes: Res<Flashes>,
    mut sold: MessageWriter<Sold>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let screen = ctx.content_rect();
    let (w, h) = (screen.width(), screen.height());
    let side = h * SQUARE_HEIGHT_PCT / 100.0;
    let left = w * SCORE_PANEL_WIDTH_PCT / 100.0 + PANEL_GAP;
    let count = portfolio.max_size as f32;
    // Squares are spread evenly over the width right of the left box.
    let spacing = ((w - PANEL_GAP - left - count * side) / (count + 1.0)).max(MIN_SPACING);
    let pad = PANEL_GAP / 2.0;
    let bg = backdrop(ctx);

    // Backing panel for the whole row, the squares sit inside it.
    let panel = rect_at(
        left,
        h - PANEL_GAP * 3.0 - side,
        w - PANEL_GAP - left,
        side + PANEL_GAP * 2.0,
    );
    card(&bg, panel, SCORE_BOX_COLOR, 0.0, 0.0);

    for slot in 0..portfolio.max_size {
        let x = left + spacing * (slot as f32 + 1.0) + side * slot as f32;
        let base = rect_at(x, panel.top() + PANEL_GAP, side, side);
        let hover = hover_amount(ctx, ("holding", slot), base);
        let hold_id = Id::new(("sell_hold", slot));

        let held = portfolio
            .slots
            .get(slot)
            .cloned()
            .flatten()
            .and_then(|holding| {
                let stock = market.get(&holding.symbol)?;
                Some((holding, stock))
            });
        let Some((holding, stock)) = &held else {
            ctx.data_mut(|d| d.remove_temp::<(f64, f64)>(hold_id));
            let square = card(&bg, base, PANEL_COLOR, 0.0, flashes.holding(slot));
            centered_lines(
                &bg,
                square.center().x,
                square.center().y - lines_height(1, STOCK_FONT_SIZE) / 2.0,
                &["Empty".to_string()],
                STOCK_FONT_SIZE,
                muted(),
            );
            continue;
        };

        let price = stock.price() as f64;
        let value = holding.shares * price;
        let profit = value - holding.shares * holding.avg_cost;
        let gain = (price / holding.avg_cost - 1.0) * 100.0;
        let up = gain >= 0.0;
        let trend = trend_color(up);
        let name_color = c32(stock_color(&holding.symbol));
        // The card is faintly tinted by whether the position is in profit.
        let tint = PANEL_COLOR.mix(&if up { STOCK_UP_COLOR } else { STOCK_DOWN_COLOR }, 0.08);
        let square = card(&bg, base, tint, hover, flashes.holding(slot));
        flashes.draw_floats(ctx, Anchor::Holding(slot), square);

        let inner_left = square.left() + pad;
        let inner_right = square.right() - pad;
        let mut y = square.top() + pad;
        bg.text(
            Pos2::new(inner_left, y),
            Align2::LEFT_TOP,
            &holding.symbol,
            FontId::proportional(SYMBOL_SIZE),
            name_color,
        );
        y += lines_height(1, SYMBOL_SIZE);
        bg.text(
            Pos2::new(inner_left, y),
            Align2::LEFT_TOP,
            format!("€{value:.2}"),
            FontId::proportional(VALUE_SIZE),
            Color32::WHITE,
        );
        bg.text(
            Pos2::new(inner_right, y + (VALUE_SIZE - DETAIL_SIZE) / 2.0),
            Align2::RIGHT_TOP,
            format!("x{:.2}", holding.shares),
            FontId::proportional(DETAIL_SIZE),
            muted(),
        );
        y += lines_height(1, VALUE_SIZE);
        let sign = if profit >= 0.0 { '+' } else { '-' };
        bg.text(
            Pos2::new(inner_left, y),
            Align2::LEFT_TOP,
            format!("{sign}€{:.2}  {gain:+.1}%", profit.abs()),
            FontId::proportional(DETAIL_SIZE),
            trend,
        );
        y += lines_height(1, DETAIL_SIZE);

        // Hold left click to sell shares (speeding up), or right click to sell one.
        let mut sells = 0;
        let mut progress = 0.0;
        place(ctx, ("sell", slot), base, |ui| {
            let response = ui.allocate_rect(base, Sense::click());
            if response.hovered() {
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                if ui.input(|i| i.pointer.button_pressed(PointerButton::Secondary)) {
                    sells += 1;
                }
            }
            let held = held_on(ui, base, &response, Id::new(("sell_down", slot)));
            let (fires, p) = hold_repeat(ui, hold_id, held);
            sells += fires;
            progress = p;
        });

        // Bottom: sell hint, replaced by a progress bar while holding.
        let track = rect_at(
            inner_left,
            square.bottom() - pad - BAR_HEIGHT,
            square.width() - 2.0 * pad,
            BAR_HEIGHT,
        );
        let hint_top = track.top() - 3.0 - lines_height(1, HINT_SIZE);
        centered_lines(
            &bg,
            square.center().x,
            hint_top,
            &["Hold or right-click: sell".to_string()],
            HINT_SIZE,
            muted(),
        );
        if progress > 0.0 && progress < 1.0 {
            let radius = CornerRadius::same(2);
            bg.rect_filled(track, radius, c32(PANEL_COLOR.darker(0.05)));
            let filled = rect_at(
                track.left(),
                track.top(),
                track.width() * progress.min(1.0),
                BAR_HEIGHT,
            );
            bg.rect_filled(filled, radius, c32(STOCK_DOWN_COLOR));
        }

        // Recent price trend in the space between the numbers and the hint.
        let recent: Vec<f32> = stock
            .history
            .iter()
            .rev()
            .take(SPARKLINE_WEEKS)
            .rev()
            .copied()
            .collect();
        let spark = rect_at(
            inner_left,
            y + 4.0,
            track.width(),
            hint_top - 4.0 - (y + 4.0),
        );
        sparkline(&bg, spark, &recent, name_color);

        for _ in 0..sells {
            if portfolio.sell_one(stock) {
                sold.write(Sold(stock.symbol.clone()));
            }
        }
    }
    Ok(())
}
