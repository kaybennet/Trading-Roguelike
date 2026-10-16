use bevy_egui::egui::{self, Align2, Color32, FontId, Painter, Pos2, Rect, Shape, Stroke, Vec2};

use super::style::{c32, card, muted};
use crate::colors::{BORDER_COLOR, SCORE_BOX_COLOR, STOCK_DOWN_COLOR, STOCK_UP_COLOR, stock_color};
use crate::consts::PANEL_GAP;
use crate::general::Date;
use crate::stocks::Stock;

/// Size the chart layout is designed for; other sizes scale from it.
const DESIGN_SIZE: Vec2 = egui::vec2(460.0, 270.0);
const MIN_FONT_SIZE: f32 = 9.5;
const WEEKS_PER_YEAR: usize = 12 * Date::WEEKS_PER_MONTH as usize;
const LABEL_SIZE: f32 = 12.0;
const GRID_LINES: usize = 4;
/// Space left of the plot for the price labels.
const AXIS_WIDTH: f32 = 56.0;
/// Space below the plot for the time labels.
const AXIS_HEIGHT: f32 = 26.0;
/// Header with the symbol, price and change.
const HEADER_HEIGHT: f32 = 62.0;

pub fn trend_color(up: bool) -> Color32 {
    c32(if up { STOCK_UP_COLOR } else { STOCK_DOWN_COLOR })
}

/// Draws `values` as a line with a fading fill below it, scaled to `lo..hi`.
fn plot(painter: &Painter, area: Rect, values: &[f32], (lo, hi): (f32, f32), color: Color32) {
    if values.len() < 2 || hi <= lo {
        return;
    }
    let points: Vec<Pos2> = values
        .iter()
        .enumerate()
        .map(|(i, &p)| {
            Pos2::new(
                area.left() + area.width() * i as f32 / (values.len() - 1) as f32,
                area.bottom() - area.height() * (p - lo) / (hi - lo),
            )
        })
        .collect();
    let mut mesh = egui::Mesh::default();
    for p in &points {
        mesh.colored_vertex(*p, color.gamma_multiply(0.3));
        mesh.colored_vertex(Pos2::new(p.x, area.bottom()), Color32::TRANSPARENT);
    }
    for i in 0..points.len() as u32 - 1 {
        let a = 2 * i;
        mesh.add_triangle(a, a + 1, a + 2);
        mesh.add_triangle(a + 1, a + 3, a + 2);
    }
    painter.add(Shape::mesh(mesh));
    painter.add(Shape::line(points, Stroke::new(2.0, color)));
}

fn range(values: &[f32]) -> (f32, f32) {
    values.iter().fold((f32::MAX, f32::MIN), |(lo, hi), &p| (lo.min(p), hi.max(p)))
}

/// A small chart of `values` filling `area`, no axes.
pub fn sparkline(painter: &Painter, area: Rect, values: &[f32], color: Color32) {
    plot(painter, area, values, range(values), color);
}

fn price_label(p: f32, span: f32) -> String {
    if span < 10.0 { format!("€{p:.2}") } else { format!("€{p:.0}") }
}

/// Draws the full price history of `stock` as a card filling `rect`, with
/// price and time axes. `avg_cost` adds a dashed line at the buy-in price.
/// Fonts and spacing scale with the size of `rect`; `hover` lifts the card.
pub fn draw_chart(painter: &Painter, rect: Rect, stock: &Stock, avg_cost: Option<f64>, hover: f32) {
    let history: Vec<f32> = stock.history.iter().copied().collect();
    let n = history.len();
    if n < 2 {
        return;
    }
    let panel = card(painter, rect, SCORE_BOX_COLOR, hover, 0.0);
    let k = (rect.width() / DESIGN_SIZE.x).min(rect.height() / DESIGN_SIZE.y).clamp(0.55, 1.8);
    let font = |size: f32| FontId::proportional((size * k).max(MIN_FONT_SIZE));
    let pad = PANEL_GAP * k.clamp(0.7, 1.3);

    let (first, last) = (history[0], history[n - 1]);
    let (low, high) = range(&history);
    let up = last >= first;
    let color = trend_color(up);
    let line_color = c32(stock_color(stock.ticker));
    painter.text(
        Pos2::new(panel.left() + pad, panel.top() + pad),
        Align2::LEFT_TOP,
        stock.ticker.name(),
        font(24.0),
        line_color,
    );
    painter.text(
        Pos2::new(panel.right() - pad, panel.top() + pad),
        Align2::RIGHT_TOP,
        format!("€{last:.2}"),
        font(24.0),
        color,
    );
    painter.text(
        Pos2::new(panel.left() + pad, panel.top() + pad + 32.0 * k),
        Align2::LEFT_TOP,
        format!(
            "{:+.1}% over {:.1} years",
            (last / first - 1.0) * 100.0,
            n as f32 / WEEKS_PER_YEAR as f32
        ),
        font(14.0),
        color,
    );
    painter.text(
        Pos2::new(panel.right() - pad, panel.top() + pad + 32.0 * k),
        Align2::RIGHT_TOP,
        format!("High €{high:.2}   Low €{low:.2}"),
        font(14.0),
        muted(),
    );

    let area = Rect::from_min_max(
        Pos2::new(panel.left() + AXIS_WIDTH * k, panel.top() + HEADER_HEIGHT * k),
        Pos2::new(panel.right() - pad, panel.bottom() - AXIS_HEIGHT * k),
    );
    // Padded price range, also covering the buy-in price.
    let (mut lo, mut hi) = (low, high);
    if let Some(cost) = avg_cost {
        lo = lo.min(cost as f32);
        hi = hi.max(cost as f32);
    }
    let margin = (hi - lo) * 0.06;
    let (lo, hi) = (lo - margin, hi + margin);
    let y_of = |p: f32| area.bottom() - area.height() * (p - lo) / (hi - lo);
    let grid = Stroke::new(1.0, c32(BORDER_COLOR).gamma_multiply(0.6));

    for i in 0..=GRID_LINES {
        let p = lo + (hi - lo) * i as f32 / GRID_LINES as f32;
        painter.hline(area.x_range(), y_of(p), grid);
        painter.text(
            Pos2::new(area.left() - 6.0 * k, y_of(p)),
            Align2::RIGHT_CENTER,
            price_label(p, hi - lo),
            font(LABEL_SIZE),
            muted(),
        );
    }
    for years_back in 0..=4 {
        let weeks_back = years_back * WEEKS_PER_YEAR;
        if weeks_back >= n {
            break;
        }
        let x = area.right() - area.width() * weeks_back as f32 / (n - 1) as f32;
        painter.vline(x, area.y_range(), grid);
        painter.text(
            Pos2::new(x, area.bottom() + 6.0 * k),
            if years_back == 0 { Align2::RIGHT_TOP } else { Align2::CENTER_TOP },
            if years_back == 0 { "now".to_string() } else { format!("-{years_back}y") },
            font(LABEL_SIZE),
            muted(),
        );
    }

    plot(painter, area, &history, (lo, hi), line_color);

    if let Some(cost) = avg_cost {
        let cy = y_of(cost as f32);
        let stroke = Stroke::new(1.0, Color32::WHITE.gamma_multiply(0.7));
        let corners = [Pos2::new(area.left(), cy), Pos2::new(area.right(), cy)];
        painter.extend(Shape::dashed_line(&corners, stroke, 6.0, 4.0));
        painter.text(
            Pos2::new(area.left() + 4.0, cy - 2.0),
            Align2::LEFT_BOTTOM,
            format!("avg €{cost:.2}"),
            font(LABEL_SIZE),
            Color32::WHITE,
        );
    }

    // Mark the latest price, the high and the low.
    let x_of = |i: usize| area.left() + area.width() * i as f32 / (n - 1) as f32;
    painter.circle_filled(Pos2::new(x_of(n - 1), y_of(last)), 4.0, line_color);
    for extreme in [low, high] {
        if let Some(i) = history.iter().position(|&p| p == extreme) {
            painter.circle_filled(Pos2::new(x_of(i), y_of(extreme)), 3.0, Color32::WHITE);
        }
    }
}
