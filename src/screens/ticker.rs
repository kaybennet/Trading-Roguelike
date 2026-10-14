use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self},
};

use super::chart::sparkline;
use super::style::{
    backdrop, c32, card, centered_lines, held_on, hold_repeat, hover_amount, lines_height, muted,
    place, rect_at,
};
use crate::colors::{ACCENT_COLOR, PANEL_COLOR, STOCK_DOWN_COLOR, STOCK_UP_COLOR, stock_color};
use crate::consts::{
    JOKER_PANEL_WIDTH_PCT, OFFER_SIZE, PANEL_GAP, SCORE_PANEL_WIDTH_PCT, STOCK_FONT_SIZE,
    TICKER_CARD_WIDTH_PCT, TICKER_FONT_SIZE, TICKER_HEIGHT_PCT, TICKER_TOP_PCT,
};
use crate::general::Portfolio;
use crate::stocks::{Market, WeeklyOffer};

/// How many of the latest weekly prices the sparkline shows.
const SPARKLINE_WEEKS: usize = 24;
/// Opacity of the price history drawn behind the card text.
const SPARKLINE_FADE: f32 = 0.2;
/// Horizontal offset of each of the two text passes that fake a bold weight.
const BOLD_OFFSET: f32 = 0.5;

/// Middle of the screen: this week's stocks, click a card to buy a share.
pub fn ticker_ui(
    mut contexts: EguiContexts,
    offer: Res<WeeklyOffer>,
    market: Res<Market>,
    mut portfolio: ResMut<Portfolio>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let screen = ctx.content_rect();
    let (w, h) = (screen.width(), screen.height());
    let band_x = w * SCORE_PANEL_WIDTH_PCT / 100.0 + PANEL_GAP;
    let band_w = w - w * JOKER_PANEL_WIDTH_PCT / 100.0 - band_x;
    let band_center_y = h * (TICKER_TOP_PCT + TICKER_HEIGHT_PCT / 2.0) / 100.0;

    let card_w = band_w * TICKER_CARD_WIDTH_PCT / 100.0;
    let spacing = (band_w - OFFER_SIZE as f32 * card_w) / (OFFER_SIZE as f32 + 1.0);
    let card_h = PANEL_GAP * 2.0
        + lines_height(3, TICKER_FONT_SIZE)
        + PANEL_GAP
        + lines_height(1, STOCK_FONT_SIZE)
        + PANEL_GAP;
    let bg = backdrop(ctx);

    for (index, symbol) in offer.0.iter().enumerate() {
        let Some(stock) = market.get(symbol) else {
            continue;
        };
        let x = band_x + spacing * (index as f32 + 1.0) + card_w * index as f32;
        let base = rect_at(x, band_center_y - card_h / 2.0, card_w, card_h);
        let affordable = portfolio.cash >= stock.price() as f64;
        let hovered = hover_amount(ctx, ("ticker", index), base);
        let hover = if affordable { hovered } else { 0.0 };
        let card_rect = card(&bg, base, PANEL_COLOR, hover, 0.0);

        let change = stock.weekly_change_pct();
        let color = c32(if change >= 0.0 {
            STOCK_UP_COLOR
        } else {
            STOCK_DOWN_COLOR
        });
        let name_color = c32(stock_color(&stock.symbol));
        let (color, name_color) = if affordable {
            (color, name_color)
        } else {
            (color.gamma_multiply(0.5), name_color.gamma_multiply(0.5))
        };

        // Faint price history behind the text.
        let recent: Vec<f32> = stock
            .history
            .iter()
            .rev()
            .take(SPARKLINE_WEEKS)
            .rev()
            .copied()
            .collect();
        let area = card_rect.shrink2(egui::vec2(PANEL_GAP, PANEL_GAP * 2.0));
        sparkline(
            &bg,
            area,
            &recent,
            name_color.gamma_multiply(SPARKLINE_FADE),
        );

        // egui has no bold weight here, so draw the text twice, offset by a fraction of a pixel.
        let lines = [format!("€{:.2}", stock.price()), format!("{change:+.1}%")];
        for dx in [-BOLD_OFFSET, BOLD_OFFSET] {
            let x = card_rect.center().x + dx;
            let below_name = centered_lines(
                &bg,
                x,
                card_rect.top() + PANEL_GAP * 2.0,
                &[stock.symbol.clone()],
                TICKER_FONT_SIZE,
                name_color,
            );
            centered_lines(&bg, x, below_name, &lines, TICKER_FONT_SIZE, color);
        }

        bg.text(
            egui::pos2(card_rect.right() - PANEL_GAP, card_rect.top() + PANEL_GAP),
            egui::Align2::RIGHT_TOP,
            format!("Div {:.1}%", stock.dividend_yield * 100.0),
            egui::FontId::proportional(STOCK_FONT_SIZE),
            muted(),
        );

        centered_lines(
            &bg,
            card_rect.center().x,
            card_rect.bottom() - PANEL_GAP - lines_height(1, STOCK_FONT_SIZE),
            &[if affordable {
                "Hold or right-click"
            } else {
                "Not enough cash"
            }
            .to_string()],
            STOCK_FONT_SIZE,
            muted(),
        );

        // The whole card is the buy button: hold left click for HOLD_DELAY_SECS
        // to buy a share and keep holding for more, or right click to buy one.
        let mut progress = 0.0;
        place(ctx, ("buy", index), base, |ui| {
            let response = ui.allocate_rect(base, egui::Sense::click());
            let mut buys = 0;
            if affordable && response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                if ui.input(|i| i.pointer.button_pressed(egui::PointerButton::Secondary)) {
                    buys += 1;
                }
            }
            let held =
                affordable && held_on(ui, base, &response, egui::Id::new(("buy_down", index)));
            let (fires, p) = hold_repeat(ui, egui::Id::new(("buy_hold", index)), held);
            buys += fires;
            progress = p;
            for _ in 0..buys {
                portfolio.buy(stock);
            }
        });

        // Progress bar along the bottom until the first share is bought.
        if progress > 0.0 && progress < 1.0 {
            let track = rect_at(
                card_rect.left() + PANEL_GAP,
                card_rect.bottom() - PANEL_GAP / 2.0 - 5.0,
                card_rect.width() - 2.0 * PANEL_GAP,
                5.0,
            );
            let radius = egui::CornerRadius::same(2);
            bg.rect_filled(track, radius, c32(PANEL_COLOR.darker(0.05)));
            let filled = rect_at(track.left(), track.top(), track.width() * progress, 5.0);
            bg.rect_filled(filled, radius, c32(ACCENT_COLOR));
        }
    }
    Ok(())
}
