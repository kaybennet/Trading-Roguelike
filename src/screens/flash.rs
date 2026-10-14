use bevy::prelude::*;

use crate::consts::{JOKER_COUNT, TRIGGER_FLASH_SECS};
use crate::general::Triggered;

/// How long the flash of each portfolio and joker slot has left.
#[derive(Resource, Default)]
pub struct Flashes {
    holdings: Vec<f32>,
    jokers: [f32; JOKER_COUNT as usize],
}

impl Flashes {
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
    mut triggered: MessageReader<Triggered>,
    mut flashes: ResMut<Flashes>,
) {
    let flashes = &mut *flashes;
    for left in flashes.holdings.iter_mut().chain(&mut flashes.jokers) {
        *left = (*left - time.delta_secs()).max(0.0);
    }
    for step in triggered.read() {
        match *step {
            Triggered::Dividend(slot) | Triggered::RepeatedDividend(slot) => {
                if flashes.holdings.len() <= slot {
                    flashes.holdings.resize(slot + 1, 0.0);
                }
                flashes.holdings[slot] = TRIGGER_FLASH_SECS;
            }
            Triggered::Joker(slot) => {
                if let Some(left) = flashes.jokers.get_mut(slot) {
                    *left = TRIGGER_FLASH_SECS;
                }
            }
        }
    }
}
