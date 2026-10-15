use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Align2, Color32, FontId, Id, Key, Order, Pos2, Sense},
};

use super::chart::draw_chart;
use super::style::{button, c32, rect_at};
use crate::colors::BG_BOTTOM_COLOR;
use crate::consts::{HOVER_SECS, PANEL_GAP, TOP_BAR_FONT_SIZE};
use crate::general::Portfolio;
use crate::stocks::{Market, Ticker};

const COLUMNS: usize = 4;
const HEADER_HEIGHT: f32 = 56.0;
const BUTTON_SIZE: egui::Vec2 = egui::vec2(120.0, 40.0);
const BUTTON_FONT_SIZE: f32 = 18.0;

/// Whether the full-screen market is open, kept in egui's memory.
pub fn open_id() -> Id {
    Id::new("market_open")
}

/// Which stock is shown large, if any.
fn focus_id() -> Id {
    Id::new("market_focus")
}

/// Full-screen view of every stock's price chart. Click a chart to enlarge it.
pub fn market_ui(
    mut contexts: EguiContexts,
    market: Res<Market>,
    portfolio: Res<Portfolio>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    if !ctx.data(|d| d.get_temp::<bool>(open_id())).unwrap_or(false) {
        return Ok(());
    }
    let screen = ctx.content_rect();
    let focus: Option<Ticker> = ctx.data(|d| d.get_temp(focus_id()));
    let mut new_focus = focus.clone();
    let mut close = false;
    if ctx.input(|i| i.key_pressed(Key::Escape)) {
        if focus.is_some() {
            new_focus = None;
        } else {
            close = true;
        }
    }

    egui::Area::new(Id::new("market"))
        .order(Order::Foreground)
        .fixed_pos(screen.min)
        .show(ctx, |ui| {
            let painter = ui.painter().clone();
            // Swallow clicks so the game underneath can't be played.
            ui.allocate_rect(screen, Sense::click());
            painter.rect_filled(screen, 0.0, c32(BG_BOTTOM_COLOR).gamma_multiply(0.97));
            painter.text(
                Pos2::new(
                    screen.left() + PANEL_GAP * 2.0,
                    screen.top() + PANEL_GAP * 2.0,
                ),
                Align2::LEFT_TOP,
                "Market",
                FontId::proportional(TOP_BAR_FONT_SIZE + 6.0),
                Color32::WHITE,
            );

            let close_rect = rect_at(
                screen.right() - PANEL_GAP * 2.0 - BUTTON_SIZE.x,
                screen.top() + PANEL_GAP * 1.5,
                BUTTON_SIZE.x,
                BUTTON_SIZE.y,
            );
            ui.scope_builder(egui::UiBuilder::new().max_rect(close_rect), |ui| {
                close |= button(ui, "Close", BUTTON_FONT_SIZE, BUTTON_SIZE).clicked();
            });
            if focus.is_some() {
                let back_rect = close_rect.translate(egui::vec2(-BUTTON_SIZE.x - PANEL_GAP, 0.0));
                ui.scope_builder(egui::UiBuilder::new().max_rect(back_rect), |ui| {
                    if button(ui, "All markets", BUTTON_FONT_SIZE, BUTTON_SIZE).clicked() {
                        new_focus = None;
                    }
                });
            }

            let content = egui::Rect::from_min_max(
                Pos2::new(
                    screen.left() + PANEL_GAP * 2.0,
                    screen.top() + HEADER_HEIGHT + PANEL_GAP,
                ),
                Pos2::new(
                    screen.right() - PANEL_GAP * 2.0,
                    screen.bottom() - PANEL_GAP * 2.0,
                ),
            );
            let avg_cost = |ticker: Ticker| {
                portfolio
                    .slots
                    .iter()
                    .flatten()
                    .find(|h| h.ticker == ticker)
                    .map(|h| h.avg_cost)
            };

            if let Some(stock) = focus.map(|t| market.get(t)) {
                draw_chart(&painter, content, stock, avg_cost(stock.ticker), 0.0);
                return;
            }
            let rows = market.stocks.len().div_ceil(COLUMNS).max(1);
            let cell_w = (content.width() - (COLUMNS - 1) as f32 * PANEL_GAP) / COLUMNS as f32;
            let cell_h = (content.height() - (rows - 1) as f32 * PANEL_GAP) / rows as f32;
            for (i, stock) in market.stocks.iter().enumerate() {
                let (row, col) = (i / COLUMNS, i % COLUMNS);
                let cell = rect_at(
                    content.left() + col as f32 * (cell_w + PANEL_GAP),
                    content.top() + row as f32 * (cell_h + PANEL_GAP),
                    cell_w,
                    cell_h,
                );
                let response =
                    ui.interact(cell, Id::new(("market_cell", stock.ticker)), Sense::click());
                let hover =
                    ui.ctx()
                        .animate_bool_with_time(response.id, response.hovered(), HOVER_SECS);
                if response.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                draw_chart(&painter, cell, stock, avg_cost(stock.ticker), hover);
                if response.clicked() {
                    new_focus = Some(stock.ticker);
                }
            }
        });

    ctx.data_mut(|d| {
        match new_focus {
            Some(ticker) if !close => {
                d.insert_temp(focus_id(), ticker);
            }
            _ => {
                d.remove_temp::<Ticker>(focus_id());
            }
        }
        if close {
            d.remove_temp::<bool>(open_id());
        }
    });
    Ok(())
}
