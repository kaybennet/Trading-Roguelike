use bevy::prelude::Color;
use bevy_egui::egui::{self, Align2, Color32, FontId, Id, LayerId, Order, Painter, Pos2, Rect};

use crate::consts::{BUTTON_COLOR, BUTTON_HOVER_COLOR, BUTTON_PRESSED_COLOR};

/// Line height as a multiple of the font size, used to stack lines by hand.
const LINE_HEIGHT: f32 = 1.25;

pub fn c32(color: Color) -> Color32 {
    let c = color.to_srgba();
    let byte = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    Color32::from_rgba_unmultiplied(byte(c.red), byte(c.green), byte(c.blue), byte(c.alpha))
}

/// Painter for the non-interactive backdrop; everything clickable is drawn above it.
pub fn backdrop(ctx: &egui::Context) -> Painter {
    ctx.layer_painter(LayerId::new(Order::Background, Id::new("backdrop")))
}

pub fn rect_at(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect::from_min_size(Pos2::new(x, y), egui::vec2(w, h))
}

/// Height of `lines` lines of text at `size`.
pub fn lines_height(lines: usize, size: f32) -> f32 {
    lines as f32 * size * LINE_HEIGHT
}

/// Draws lines centred on `center_x`, the first one at `top`. Returns the bottom edge.
pub fn centered_lines(
    painter: &Painter,
    center_x: f32,
    top: f32,
    lines: &[String],
    size: f32,
    color: Color32,
) -> f32 {
    let mut y = top;
    for line in lines {
        painter.text(
            Pos2::new(center_x, y),
            Align2::CENTER_TOP,
            line,
            FontId::proportional(size),
            color,
        );
        y += size * LINE_HEIGHT;
    }
    y
}

/// Hosts interactive widgets in `rect`. The area has no background of its own.
pub fn place(
    ctx: &egui::Context,
    id: impl std::hash::Hash + std::fmt::Debug,
    rect: Rect,
    add: impl FnOnce(&mut egui::Ui),
) {
    egui::Area::new(Id::new(id))
        .order(Order::Middle)
        .fixed_pos(rect.min)
        .show(ctx, |ui| {
            ui.scope_builder(egui::UiBuilder::new().max_rect(rect), add);
        });
}

/// A button with the game's hover and press colours.
pub fn button(ui: &mut egui::Ui, label: &str, font_size: f32, size: egui::Vec2) -> egui::Response {
    ui.scope(|ui| {
        let widgets = &mut ui.visuals_mut().widgets;
        widgets.inactive.weak_bg_fill = c32(BUTTON_COLOR);
        widgets.hovered.weak_bg_fill = c32(BUTTON_HOVER_COLOR);
        widgets.active.weak_bg_fill = c32(BUTTON_PRESSED_COLOR);
        widgets.hovered.bg_stroke = egui::Stroke::NONE;
        widgets.active.bg_stroke = egui::Stroke::NONE;
        let text = egui::RichText::new(label)
            .size(font_size)
            .color(Color32::WHITE);
        ui.add_sized(size, egui::Button::new(text))
    })
    .inner
}
