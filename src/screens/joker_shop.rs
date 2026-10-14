use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Color32, Id, Order, Painter, Rect, Ui},
};

use super::style::{button, c32, centered_lines, lines_height, rect_at};
use crate::consts::{
    GAME_OVER_BG_COLOR, JOKER_COUNT, JOKER_FONT_SIZE, MODIFIER_COLOR, PANEL_COLOR, PANEL_GAP,
    SCORE_FONT_SIZE, SHOP_OFFER_SIZE, TICKER_FONT_SIZE,
};
use crate::general::{AppState, GameState, Portfolio};
use crate::shop::{Card, ShopOffer};

const SHOP_COLUMNS: usize = 2;
const CONTINUE_SIZE: egui::Vec2 = egui::vec2(180.0, 48.0);

/// Draws one shop card with a Buy button; returns true when Buy is clicked.
fn shop_card(
    ui: &mut Ui,
    painter: &Painter,
    card: Rect,
    color: Color,
    lines: [String; 3],
    buyable: bool,
) -> bool {
    painter.rect_filled(card, 0.0, c32(color));
    centered_lines(
        painter,
        card.center().x,
        card.top() + PANEL_GAP / 2.0,
        &lines,
        JOKER_FONT_SIZE,
        Color32::WHITE,
    );
    let size = egui::vec2(card.width() - PANEL_GAP * 2.0, SCORE_FONT_SIZE + 14.0);
    let rect = Rect::from_min_size(
        egui::pos2(
            card.center().x - size.x / 2.0,
            card.bottom() - PANEL_GAP / 2.0 - size.y,
        ),
        size,
    );
    ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
        ui.add_enabled_ui(buyable, |ui| {
            button(ui, "Buy", SCORE_FONT_SIZE, size).clicked()
        })
        .inner
    })
    .inner
}

/// Full-screen shop shown after the monthly debt is paid: buy jokers and
/// modifier cards, then continue.
pub fn joker_shop_ui(
    mut contexts: EguiContexts,
    mut offer: ResMut<ShopOffer>,
    mut portfolio: ResMut<Portfolio>,
    mut state: ResMut<GameState>,
    mut next_state: ResMut<NextState<AppState>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let screen = ctx.content_rect();
    let (w, h) = (screen.width(), screen.height());
    // Header, card grid and Continue form one block centred on screen.
    let card_w = w * 0.2;
    let card_h = lines_height(3, JOKER_FONT_SIZE) + SCORE_FONT_SIZE + 14.0 + PANEL_GAP * 1.5;
    let header_h = lines_height(2, TICKER_FONT_SIZE);
    let grid_w = SHOP_COLUMNS as f32 * card_w + (SHOP_COLUMNS - 1) as f32 * PANEL_GAP;
    let rows = SHOP_OFFER_SIZE.div_ceil(SHOP_COLUMNS) as f32;
    let grid_h = rows * card_h + (rows - 1.0) * PANEL_GAP;
    let block_h = header_h + grid_h + CONTINUE_SIZE.y + PANEL_GAP * 4.0;
    let header_top = ((h - block_h) / 2.0).max(PANEL_GAP);
    let grid_left = (w - grid_w) / 2.0;
    let grid_top = header_top + header_h + PANEL_GAP * 2.0;
    let mut bought = None;
    let mut done = false;

    egui::Area::new(Id::new("joker_shop"))
        .order(Order::Foreground)
        .fixed_pos(screen.min)
        .show(ctx, |ui| {
            let painter = ui.painter().clone();
            // Swallow clicks so the game underneath can't be played.
            ui.allocate_rect(screen, egui::Sense::click());
            painter.rect_filled(screen, 0.0, c32(GAME_OVER_BG_COLOR));
            centered_lines(
                &painter,
                w / 2.0,
                header_top,
                &[
                    "Shop".to_string(),
                    format!(
                        "Cash: €{:.2}   Jokers: {}/{}",
                        portfolio.cash,
                        portfolio.jokers.len(),
                        JOKER_COUNT
                    ),
                ],
                TICKER_FONT_SIZE,
                Color32::WHITE,
            );
            for (index, &card) in offer.0.iter().enumerate() {
                let (row, col) = (index / SHOP_COLUMNS, index % SHOP_COLUMNS);
                let rect = rect_at(
                    grid_left + col as f32 * (card_w + PANEL_GAP),
                    grid_top + row as f32 * (card_h + PANEL_GAP),
                    card_w,
                    card_h,
                );
                let color = match card {
                    Card::Joker(_) => PANEL_COLOR,
                    Card::Modifier(_) => MODIFIER_COLOR,
                };
                let lines = [
                    card.name().to_string(),
                    format!("€{:.2}", card.price()),
                    card.description().to_string(),
                ];
                if shop_card(ui, &painter, rect, color, lines, card.can_buy(&portfolio)) {
                    bought = Some(index);
                }
            }
            let rect = Rect::from_min_size(
                egui::pos2(
                    w / 2.0 - CONTINUE_SIZE.x / 2.0,
                    grid_top + grid_h + PANEL_GAP * 2.0,
                ),
                CONTINUE_SIZE,
            );
            ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                done = button(ui, "Continue", SCORE_FONT_SIZE, CONTINUE_SIZE).clicked();
            });
        });

    if let Some(index) = bought {
        offer.buy(index, &mut portfolio, &mut state);
    }
    if done {
        next_state.set(AppState::Playing);
    }
    Ok(())
}
