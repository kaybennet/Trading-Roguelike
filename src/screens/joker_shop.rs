use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Color32, Id, Order, Painter, Rect, Ui},
};

use super::style::{
    button, c32, card, centered_lines, lines_height, muted, rarity_outline, rect_at, wrapped_text,
};
use crate::colors::{ACCENT_COLOR, GAME_OVER_BG_COLOR, MODIFIER_COLOR, PANEL_COLOR, rarity_color};
use crate::consts::{
    HOVER_SECS, JOKER_COUNT, PANEL_GAP, SCORE_FONT_SIZE, SHOP_OFFER_SIZE, TICKER_FONT_SIZE,
};
use crate::general::{AppState, GameState, Portfolio};
use crate::shop::{Card, ShopOffer};

const SHOP_COLUMNS: usize = 2;
const CONTINUE_SIZE: egui::Vec2 = egui::vec2(180.0, 48.0);
const TAG_FONT_SIZE: f32 = 13.0;
const NAME_FONT_SIZE: f32 = 22.0;
const PRICE_FONT_SIZE: f32 = 20.0;
const DESCRIPTION_FONT_SIZE: f32 = 16.0;
const HINT_FONT_SIZE: f32 = 14.0;

/// Draws one shop card; returns true when it is clicked while buyable.
fn shop_card(
    ui: &mut Ui,
    painter: &Painter,
    base: Rect,
    offered: Card,
    hint: &str,
    buyable: bool,
) -> bool {
    let response = ui.interact(
        base,
        Id::new(("shop_card", offered.name())),
        egui::Sense::click(),
    );
    let hover =
        ui.ctx()
            .animate_bool_with_time(response.id, buyable && response.hovered(), HOVER_SECS);
    if buyable && response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    let (color, kind) = match offered {
        Card::Joker(_) => (PANEL_COLOR, "JOKER"),
        Card::Modifier(_) => (MODIFIER_COLOR, "MODIFIER"),
    };
    let rarity = offered.rarity();
    let tag = format!("{kind} · {}", rarity.name().to_uppercase());
    let rect = card(painter, base, color, hover, 0.0);
    rarity_outline(painter, rect, rarity);
    let x = rect.center().x;
    let wrap = rect.width() - PANEL_GAP * 2.0;
    let mut y = rect.top() + PANEL_GAP;
    y = centered_lines(
        painter,
        x,
        y,
        &[tag],
        TAG_FONT_SIZE,
        c32(rarity_color(rarity)),
    );
    y = centered_lines(
        painter,
        x,
        y,
        &[offered.name().to_string()],
        NAME_FONT_SIZE,
        Color32::WHITE,
    );
    let price_color = if buyable { c32(ACCENT_COLOR) } else { muted() };
    y = centered_lines(
        painter,
        x,
        y,
        &[format!("€{:.2}", offered.price())],
        PRICE_FONT_SIZE,
        price_color,
    );
    wrapped_text(
        painter,
        x,
        y + 2.0,
        offered.description(),
        DESCRIPTION_FONT_SIZE,
        Color32::WHITE,
        wrap,
    );
    centered_lines(
        painter,
        x,
        rect.bottom() - PANEL_GAP - lines_height(1, HINT_FONT_SIZE),
        &[hint.to_string()],
        HINT_FONT_SIZE,
        muted(),
    );
    buyable && response.clicked()
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
    let card_h = PANEL_GAP * 3.0
        + lines_height(1, TAG_FONT_SIZE)
        + lines_height(1, NAME_FONT_SIZE)
        + lines_height(1, PRICE_FONT_SIZE)
        + lines_height(2, DESCRIPTION_FONT_SIZE)
        + lines_height(1, HINT_FONT_SIZE);
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
            for (index, &offered) in offer.0.iter().enumerate() {
                let (row, col) = (index / SHOP_COLUMNS, index % SHOP_COLUMNS);
                let rect = rect_at(
                    grid_left + col as f32 * (card_w + PANEL_GAP),
                    grid_top + row as f32 * (card_h + PANEL_GAP),
                    card_w,
                    card_h,
                );
                let buyable = offered.can_buy(&portfolio);
                let hint = if buyable {
                    "Click to buy"
                } else if portfolio.cash < offered.price() {
                    "Not enough cash"
                } else {
                    "No free joker slot"
                };
                if shop_card(ui, &painter, rect, offered, hint, buyable) {
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
