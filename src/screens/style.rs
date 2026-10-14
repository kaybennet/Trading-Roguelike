use std::hash::Hash;

use bevy::color::Mix;
use bevy::prelude::{Color, Local, Result};
use bevy_egui::{
    EguiContexts,
    egui::{
        self, Align, Align2, Color32, CornerRadius, CursorIcon, FontId, Id, LayerId, Order,
        Painter, Pos2, Rect, Shape, Stroke, StrokeKind, text::LayoutJob,
    },
};

use crate::colors::rarity_color;
use crate::colors::{
    ACCENT_COLOR, BG_BOTTOM_COLOR, BG_TOP_COLOR, BORDER_COLOR, BUTTON_COLOR, BUTTON_HOVER_COLOR,
    BUTTON_PRESSED_COLOR, MUTED_COLOR, PANEL_COLOR, TRIGGER_FLASH_COLOR,
};
use crate::consts::{
    CORNER_RADIUS, DESIGN_HEIGHT, DESIGN_WIDTH, HOLD_ACCEL_SECS, HOLD_DELAY_SECS,
    HOLD_REPEAT_MIN_SECS, HOLD_REPEAT_START_SECS, HOVER_LIFT, HOVER_SECS,
};
use crate::rarity::Rarity;

/// Line height as a multiple of the font size, used to stack lines by hand.
const LINE_HEIGHT: f32 = 1.25;

pub fn c32(color: Color) -> Color32 {
    let c = color.to_srgba();
    let byte = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    Color32::from_rgba_unmultiplied(byte(c.red), byte(c.green), byte(c.blue), byte(c.alpha))
}

/// Dark, rounded look for everything egui draws itself (tooltips, buttons).
pub fn apply_theme(mut contexts: EguiContexts, mut done: Local<bool>) -> Result {
    let ctx = contexts.ctx_mut()?;
    // Zoom so the window always spans at least the design size in points; the
    // percentage-based layout then only has to cope with the aspect ratio.
    let zoom = ctx.zoom_factor();
    let rect = ctx.content_rect();
    let fit = (rect.width() * zoom / DESIGN_WIDTH)
        .min(rect.height() * zoom / DESIGN_HEIGHT)
        .clamp(0.5, 3.0);
    if (fit - zoom).abs() > 0.01 {
        ctx.set_zoom_factor(fit);
    }
    if *done {
        return Ok(());
    }
    let mut visuals = egui::Visuals::dark();
    visuals.window_fill = c32(PANEL_COLOR);
    visuals.window_stroke = Stroke::new(1.0, c32(BORDER_COLOR));
    visuals.window_corner_radius = CornerRadius::same(CORNER_RADIUS);
    ctx.set_visuals(visuals);
    *done = true;
    Ok(())
}

/// Painter for the non-interactive backdrop; everything clickable is drawn above it.
pub fn backdrop(ctx: &egui::Context) -> Painter {
    ctx.layer_painter(LayerId::new(Order::Background, Id::new("backdrop")))
}

/// Fills the screen with the vertical background gradient.
pub fn draw_background(painter: &Painter, screen: Rect) {
    let (top, bottom) = (c32(BG_TOP_COLOR), c32(BG_BOTTOM_COLOR));
    let mut mesh = egui::Mesh::default();
    mesh.colored_vertex(screen.left_top(), top);
    mesh.colored_vertex(screen.right_top(), top);
    mesh.colored_vertex(screen.right_bottom(), bottom);
    mesh.colored_vertex(screen.left_bottom(), bottom);
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(0, 2, 3);
    painter.add(Shape::mesh(mesh));
}

pub fn rect_at(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect::from_min_size(Pos2::new(x, y), egui::vec2(w, h))
}

/// Height of `lines` lines of text at `size`.
pub fn lines_height(lines: usize, size: f32) -> f32 {
    lines as f32 * size * LINE_HEIGHT
}

/// How hovered `rect` is, from 0 to 1, eased over `HOVER_SECS`. Anything covered
/// by an overlay (shop, game over) doesn't count as hovered.
pub fn hover_amount(ctx: &egui::Context, id: impl Hash + std::fmt::Debug, rect: Rect) -> f32 {
    let hovered = ctx.pointer_hover_pos().is_some_and(|p| {
        rect.contains(p) && ctx.layer_id_at(p).is_none_or(|l| l.order <= Order::Middle)
    });
    ctx.animate_bool_with_time(Id::new(id), hovered, HOVER_SECS)
}

/// Draws a rounded card with shadow and border. Hovering lifts it and lights
/// the border, `flash` tints it towards the trigger colour. Returns the
/// (lifted) rect so contents can follow it.
pub fn card(painter: &Painter, rect: Rect, color: Color, hover: f32, flash: f32) -> Rect {
    let radius = CornerRadius::same(CORNER_RADIUS);
    let rect = rect.translate(egui::vec2(0.0, -HOVER_LIFT * hover));
    let shadow = Color32::from_black_alpha((70.0 + 50.0 * hover) as u8);
    painter.rect_filled(
        rect.translate(egui::vec2(0.0, 3.0 + 3.0 * hover)),
        radius,
        shadow,
    );
    let fill = color
        .mix(&Color::WHITE, 0.08 * hover)
        .mix(&TRIGGER_FLASH_COLOR, flash);
    painter.rect_filled(rect, radius, c32(fill));
    let border = c32(BORDER_COLOR.mix(&ACCENT_COLOR, hover));
    painter.rect_stroke(rect, radius, Stroke::new(1.0, border), StrokeKind::Inside);
    rect
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

/// Draws `text` centred on `center_x`, wrapped at `wrap` width. Returns the bottom edge.
pub fn wrapped_text(
    painter: &Painter,
    center_x: f32,
    top: f32,
    text: &str,
    size: f32,
    color: Color32,
    wrap: f32,
) -> f32 {
    let mut job = LayoutJob::simple(text.to_owned(), FontId::proportional(size), color, wrap);
    job.halign = Align::Center;
    let galley = painter.layout_job(job);
    let bottom = top + galley.size().y;
    painter.galley(
        Pos2::new(center_x - galley.rect.center().x, top),
        galley,
        color,
    );
    bottom
}

/// Muted colour for secondary text.
pub fn muted() -> Color32 {
    c32(MUTED_COLOR)
}

/// Whether the left button has been held down since it was pressed on `rect`.
/// egui's own pressed state ends after its click duration (~0.8s), too early
/// for hold-to-buy and hold-to-sell, so this follows the raw pointer instead.
pub fn held_on(ui: &egui::Ui, rect: Rect, response: &egui::Response, id: Id) -> bool {
    let (down, pressed, pos) = ui.input(|i| {
        (
            i.pointer.primary_down(),
            i.pointer.button_pressed(egui::PointerButton::Primary),
            i.pointer.hover_pos(),
        )
    });
    let inside = pos.is_some_and(|p| rect.contains(p));
    let was_held = ui.data(|d| d.get_temp::<bool>(id)).unwrap_or(false);
    let held = down && inside && (was_held || (pressed && response.hovered()));
    ui.data_mut(|d| {
        if held {
            d.insert_temp(id, true);
        } else {
            d.remove_temp::<bool>(id);
        }
    });
    held
}

/// Hold-to-repeat timing while `active`. Returns how many times to fire this frame
/// and the progress (0..1) of the initial delay. After `HOLD_DELAY_SECS` it fires,
/// then keeps firing at an interval that shrinks the longer the hold lasts.
pub fn hold_repeat(ui: &egui::Ui, id: Id, active: bool) -> (u32, f32) {
    if !active {
        ui.data_mut(|d| d.remove_temp::<(f64, f64)>(id));
        return (0, 0.0);
    }
    let now = ui.input(|i| i.time);
    let (start, mut next) = ui
        .data(|d| d.get_temp::<(f64, f64)>(id))
        .unwrap_or((now, now + HOLD_DELAY_SECS));
    let mut fires = 0;
    while now >= next && fires < 100 {
        fires += 1;
        let t = ((next - start - HOLD_DELAY_SECS) / HOLD_ACCEL_SECS).clamp(0.0, 1.0);
        next += HOLD_REPEAT_START_SECS + (HOLD_REPEAT_MIN_SECS - HOLD_REPEAT_START_SECS) * t;
    }
    ui.data_mut(|d| d.insert_temp(id, (start, next)));
    (fires, ((now - start) / HOLD_DELAY_SECS).min(1.0) as f32)
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

/// A rounded button that fades between the game's hover and press colours.
/// Greyed out and unclickable when the `ui` is disabled.
pub fn button(ui: &mut egui::Ui, label: &str, font_size: f32, size: egui::Vec2) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let enabled = ui.is_enabled();
    let hover =
        ui.ctx()
            .animate_bool_with_time(response.id, enabled && response.hovered(), HOVER_SECS);
    let fill = if !enabled {
        Color::srgb(0.18, 0.2, 0.26)
    } else if response.is_pointer_button_down_on() {
        BUTTON_PRESSED_COLOR
    } else {
        BUTTON_COLOR.mix(&BUTTON_HOVER_COLOR, hover)
    };
    let text_color = if enabled { Color32::WHITE } else { muted() };
    ui.painter()
        .rect_filled(rect, CornerRadius::same(CORNER_RADIUS - 2), c32(fill));
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        label,
        FontId::proportional(font_size),
        text_color,
    );
    if enabled && response.hovered() {
        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
    }
    response
}

/// Outlines `rect` in the colour of a rarity tier.
pub fn rarity_outline(painter: &Painter, rect: Rect, rarity: Rarity) {
    painter.rect_stroke(
        rect,
        CornerRadius::same(CORNER_RADIUS),
        Stroke::new(2.0, c32(rarity_color(rarity))),
        StrokeKind::Inside,
    );
}
