use bevy::prelude::*;

use bevy_egui::egui::{self, Align2, Color32, FontId, Id, LayerId, Order};

use crate::colors::STOCK_UP_COLOR;
use crate::consts::{FLOAT_FONT_SIZE, FLOAT_RISE, FLOAT_SECS, JOKER_COUNT, TRIGGER_FLASH_SECS};
use crate::general::{Boost, GameEvent, Triggered};

use super::style::c32;

/// What a floating amount is attached to.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    Holding(usize),
    Joker(usize),
}

struct Float {
    anchor: Anchor,
    text: String,
    age: f32,
    /// Cash total and how many payouts it sums, so repeats can be merged into it.
    cash: Option<(f64, f64)>,
}

/// How long the flash of each portfolio and joker slot has left.
#[derive(Resource, Default)]
pub struct Flashes {
    holdings: Vec<f32>,
    jokers: [f32; JOKER_COUNT as usize],
    floats: Vec<Float>,
}

impl Flashes {
    /// Draws the "+€x" amounts rising and fading above `rect`, on top of everything.
    pub fn draw_floats(&self, ctx: &egui::Context, anchor: Anchor, rect: egui::Rect) {
        let painter = ctx.layer_painter(LayerId::new(Order::Tooltip, Id::new("floats")));
        for float in self.floats.iter().filter(|f| f.anchor == anchor) {
            let t = float.age / FLOAT_SECS;
            let rise = FLOAT_RISE * (1.0 - (1.0 - t).powi(2));
            let alpha = (1.0 - t * t).clamp(0.0, 1.0);
            let pos = egui::pos2(rect.center().x, rect.top() - rise);
            let text = &float.text;
            let font = FontId::proportional(FLOAT_FONT_SIZE);
            let shadow = Color32::from_black_alpha((alpha * 200.0) as u8);
            painter.text(
                pos + egui::vec2(1.0, 1.0),
                Align2::CENTER_BOTTOM,
                text,
                font.clone(),
                shadow,
            );
            let color = c32(STOCK_UP_COLOR).gamma_multiply(alpha);
            painter.text(pos, Align2::CENTER_BOTTOM, text, font, color);
        }
    }

    fn flash_holding(&mut self, slot: usize) {
        if self.holdings.len() <= slot {
            self.holdings.resize(slot + 1, 0.0);
        }
        self.holdings[slot] = TRIGGER_FLASH_SECS;
    }

    /// Brightness of a portfolio slot's flash, from 0 (off) to 1 (just triggered).
    pub fn holding(&self, slot: usize) -> f32 {
        self.holdings
            .get(slot)
            .map_or(0.0, |left| left / TRIGGER_FLASH_SECS)
    }

    /// Brightness of a joker slot's flash, from 0 (off) to 1 (just triggered).
    pub fn joker(&self, slot: usize) -> f32 {
        self.jokers
            .get(slot)
            .map_or(0.0, |left| left / TRIGGER_FLASH_SECS)
    }
}

/// Fades the flashes and lights up whatever just triggered.
pub fn update_flashes(
    time: Res<Time>,
    mut events: MessageReader<GameEvent>,
    mut flashes: ResMut<Flashes>,
) {
    let flashes = &mut *flashes;
    for float in &mut flashes.floats {
        float.age += time.delta_secs();
    }
    flashes.floats.retain(|f| f.age < FLOAT_SECS);
    let events: Vec<GameEvent> = events.read().cloned().collect();
    let gains = events.iter().filter_map(|e| match e {
        GameEvent::CashGained(gain) => Some(gain),
        _ => None,
    });
    let share_gains = events.iter().filter_map(|e| match e {
        GameEvent::SharesGained(gain) => Some(gain),
        _ => None,
    });
    let triggered = events.iter().filter_map(|e| match e {
        GameEvent::Triggered(step) => Some(step),
        _ => None,
    });
    for gain in gains {
        let anchor = match gain.source {
            Triggered::Dividend(slot) | Triggered::DividendBoost { slot, .. } => {
                Anchor::Holding(slot)
            }
            Triggered::Joker(slot) => Anchor::Joker(slot),
        };
        // A boost adds to the float of the dividend it boosts.
        let boost = match gain.source {
            Triggered::DividendBoost { boost, .. } => Some(boost),
            _ => None,
        };
        let merge = boost
            .is_some()
            .then(|| {
                flashes
                    .floats
                    .iter_mut()
                    .rfind(|f| f.anchor == anchor && f.cash.is_some())
            })
            .flatten();
        if let Some(float) = merge
            && let Some((total, times)) = &mut float.cash
        {
            *total += gain.amount;
            if let Some(Boost::Mul(factor)) = boost {
                *times *= factor;
            }
            float.text = if *times > 1.0 {
                format!("+€{total:.2} x{times}")
            } else {
                format!("+€{total:.2}")
            };
            float.age = 0.0;
            continue;
        }
        flashes.floats.push(Float {
            anchor,
            text: format!("+€{:.2}", gain.amount),
            age: 0.0,
            cash: Some((gain.amount, 1.0)),
        });
    }
    for gain in share_gains {
        flashes.floats.push(Float {
            anchor: Anchor::Holding(gain.slot),
            text: format!("+{} share", gain.amount),
            age: 0.0,
            cash: None,
        });
        flashes.flash_holding(gain.slot);
    }
    for left in flashes.holdings.iter_mut().chain(&mut flashes.jokers) {
        *left = (*left - time.delta_secs()).max(0.0);
    }
    for step in triggered {
        match *step {
            Triggered::Dividend(slot) => {
                flashes.flash_holding(slot);
            }
            Triggered::DividendBoost { slot, joker, .. } => {
                flashes.flash_holding(slot);
                // The joker of this step lights up.
                if let Some(left) = flashes.jokers.get_mut(joker) {
                    *left = TRIGGER_FLASH_SECS;
                }
            }
            Triggered::Joker(slot) => {
                if let Some(left) = flashes.jokers.get_mut(slot) {
                    *left = TRIGGER_FLASH_SECS;
                }
            }
        }
    }
}
