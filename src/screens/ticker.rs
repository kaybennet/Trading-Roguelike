use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use super::style::{backdrop, button, c32, centered_lines, lines_height, place, rect_at};
use crate::consts::{
    JOKER_PANEL_WIDTH_PCT, OFFER_SIZE, PANEL_COLOR, PANEL_GAP, SCORE_PANEL_WIDTH_PCT,
    STOCK_DOWN_COLOR, STOCK_UP_COLOR, TICKER_CARD_WIDTH_PCT, TICKER_FONT_SIZE, TICKER_HEIGHT_PCT,
    TICKER_TOP_PCT,
};
use crate::general::Portfolio;
use crate::stocks::{Market, WeeklyOffer};

/// Middle of the screen: this week's stocks, each with a Buy button.
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
    let button_size = egui::vec2(card_w - PANEL_GAP * 4.0, TICKER_FONT_SIZE + 16.0);
    let card_h = PANEL_GAP * 2.0
        + lines_height(3, TICKER_FONT_SIZE)
        + PANEL_GAP
        + button_size.y
        + PANEL_GAP * 2.0;
    let bg = backdrop(ctx);

    for (index, symbol) in offer.0.iter().enumerate() {
        let Some(stock) = market.get(symbol) else {
            continue;
        };
        let x = band_x + spacing * (index as f32 + 1.0) + card_w * index as f32;
        let card = rect_at(x, band_center_y - card_h / 2.0, card_w, card_h);
        bg.rect_filled(card, 0.0, c32(PANEL_COLOR));

        let change = stock.weekly_change_pct();
        let color = c32(if change >= 0.0 {
            STOCK_UP_COLOR
        } else {
            STOCK_DOWN_COLOR
        });
        let lines = [
            stock.symbol.clone(),
            format!("€{:.2}", stock.price()),
            format!("{change:+.1}%"),
        ];
        centered_lines(
            &bg,
            card.center().x,
            card.top() + PANEL_GAP * 2.0,
            &lines,
            TICKER_FONT_SIZE,
            color,
        );

        let button_rect = egui::Rect::from_min_size(
            egui::pos2(
                card.center().x - button_size.x / 2.0,
                card.bottom() - PANEL_GAP * 2.0 - button_size.y,
            ),
            button_size,
        );
        place(ctx, ("buy", index), button_rect, |ui| {
            if button(ui, "Buy", TICKER_FONT_SIZE, button_size).clicked() {
                portfolio.buy(stock);
            }
        });
    }
    Ok(())
}
