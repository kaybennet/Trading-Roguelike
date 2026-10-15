use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{
        self, Align2, Color32, CornerRadius, CursorIcon, FontId, Id, LayerId, Order, Pos2, Sense,
        Shape, Stroke, StrokeKind,
    },
};

use super::flash::{Anchor, Flashes};
use super::style::{backdrop, c32, card, hover_amount, muted, place, rarity_outline, rect_at};
use crate::colors::{ACCENT_COLOR, BORDER_COLOR, PANEL_COLOR, SCORE_BOX_COLOR};
use crate::consts::{
    CORNER_RADIUS, JOKER_COUNT, JOKER_FONT_SIZE, JOKER_HEIGHT_PCT, JOKER_PANEL_WIDTH_PCT,
    JOKER_SELL_FRACTION, PANEL_GAP, TICKER_HEIGHT_PCT, TICKER_TOP_PCT,
};
use crate::general::Portfolio;
use crate::jokers::Joker;

/// Draws one joker's name and its position number inside `slot`.
fn joker_label(painter: &egui::Painter, slot: egui::Rect, index: usize, joker: Joker) {
    painter.text(
        slot.center(),
        Align2::CENTER_CENTER,
        joker.name(),
        FontId::proportional(JOKER_FONT_SIZE),
        Color32::WHITE,
    );
    painter.text(
        slot.left_top() + egui::vec2(PANEL_GAP / 2.0, PANEL_GAP / 2.0),
        Align2::LEFT_TOP,
        index + 1,
        FontId::proportional(JOKER_FONT_SIZE * 0.7),
        muted(),
    );
}

/// Right side of the screen: a column of joker slots, vertically centred on the
/// same line as the weekly stock ticker. Jokers can be dragged to reorder them.
pub fn jokers_ui(
    mut contexts: EguiContexts,
    mut portfolio: ResMut<Portfolio>,
    flashes: Res<Flashes>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let screen = ctx.content_rect();
    let (w, h) = (screen.width(), screen.height());
    let column_w = w * JOKER_PANEL_WIDTH_PCT / 100.0;
    let slot_w = column_w - 2.0 * PANEL_GAP;
    let slot_h = h * JOKER_HEIGHT_PCT / 100.0;
    let count = JOKER_COUNT as usize;
    let total = count as f32 * slot_h + (count - 1) as f32 * PANEL_GAP;
    let top = h * (TICKER_TOP_PCT + TICKER_HEIGHT_PCT / 2.0) / 100.0 - total / 2.0;
    let slot_rect = |n: usize| {
        rect_at(
            w - column_w + PANEL_GAP,
            top + n as f32 * (slot_h + PANEL_GAP),
            slot_w,
            slot_h,
        )
    };
    let bg = backdrop(ctx);

    // The joker being dragged (its slot index) is kept in egui's memory.
    let drag_id = Id::new("joker_drag");
    let mut dragging: Option<usize> = ctx.data(|d| d.get_temp(drag_id));
    let pointer = ctx.pointer_interact_pos();
    let target = pointer.map(|p| {
        let row = ((p.y - top + PANEL_GAP / 2.0) / (slot_h + PANEL_GAP)).floor();
        row.clamp(0.0, (count - 1) as f32) as usize
    });
    if let Some(from) = dragging
        && !ctx.input(|i| i.pointer.any_down())
    {
        if let Some(to) = target {
            portfolio.move_joker(from, to);
        }
        ctx.data_mut(|d| d.remove_temp::<usize>(drag_id));
        dragging = None;
    }

    let mut started = None;
    let mut sell = None;
    for n in 0..count {
        let rect = slot_rect(n);
        let joker = portfolio.jokers.get(n).copied();
        let hover = if dragging.is_none() && joker.is_some() {
            hover_amount(ctx, ("joker", n), rect)
        } else {
            0.0
        };
        match joker {
            None => {
                let corners = [
                    rect.left_top(),
                    rect.right_top(),
                    rect.right_bottom(),
                    rect.left_bottom(),
                    rect.left_top(),
                ];
                let stroke = Stroke::new(1.0, c32(BORDER_COLOR));
                bg.extend(Shape::dashed_line(&corners, stroke, 6.0, 4.0));
                bg.text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    "Empty",
                    FontId::proportional(JOKER_FONT_SIZE * 0.8),
                    muted(),
                );
            }
            // Where the dragged joker comes from: a dim hole.
            Some(_) if dragging == Some(n) => {
                card(&bg, rect, SCORE_BOX_COLOR, 0.0, 0.0);
            }
            Some(joker) => {
                let lifted = card(&bg, rect, PANEL_COLOR, hover, flashes.joker(n));
                rarity_outline(&bg, lifted, joker.rarity());
                joker_label(&bg, lifted, n, joker);
                flashes.draw_floats(ctx, Anchor::Joker(n), lifted);
            }
        }
        if dragging.is_some() && dragging != Some(n) && target == Some(n) {
            bg.rect_stroke(
                rect.expand(2.0),
                CornerRadius::same(CORNER_RADIUS),
                Stroke::new(2.0, c32(ACCENT_COLOR)),
                StrokeKind::Outside,
            );
        }
        if let Some(joker) = joker {
            place(ctx, ("joker_slot", n), rect, |ui| {
                let response = ui.allocate_rect(rect, Sense::click_and_drag());
                if response.secondary_clicked() {
                    sell = Some(n);
                }
                if response.drag_started() {
                    started = Some(n);
                }
                if dragging.is_none() {
                    response
                        .on_hover_cursor(CursorIcon::Grab)
                        .on_hover_ui(|ui| {
                            ui.strong(joker.name());
                            ui.label(joker.description());
                            ui.label(
                                egui::RichText::new(format!(
                                    "Drag to reorder, right-click to sell for €{:.2}",
                                    joker.price() * JOKER_SELL_FRACTION
                                ))
                                .color(muted()),
                            );
                        });
                }
            });
        }
    }
    if let Some(n) = sell
        && let Some(joker) = portfolio.jokers.get(n).copied()
    {
        portfolio.jokers.remove(n);
        portfolio.cash += joker.price() * JOKER_SELL_FRACTION;
    }
    if let Some(n) = started {
        ctx.data_mut(|d| d.insert_temp(drag_id, n));
    }

    // The dragged joker follows the pointer above everything else.
    if let (Some(from), Some(p)) = (dragging, pointer)
        && let Some(&joker) = portfolio.jokers.get(from)
    {
        ctx.set_cursor_icon(CursorIcon::Grabbing);
        let ghost = slot_rect(from);
        let ghost = egui::Rect::from_center_size(Pos2::new(ghost.center().x, p.y), ghost.size());
        let painter = ctx.layer_painter(LayerId::new(Order::Tooltip, Id::new("joker_ghost")));
        let lifted = card(&painter, ghost, PANEL_COLOR, 1.0, 0.0);
        rarity_outline(&painter, lifted, joker.rarity());
        joker_label(&painter, lifted, from, joker);
    }
    Ok(())
}
