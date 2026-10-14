use bevy::prelude::*;

use crate::general::{Portfolio, Sequence, SequenceSet, Triggered};

pub struct ListenersPlugin;

impl Plugin for ListenersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, retrigger_dividends.after(SequenceSet));
    }
}

/// Jokers that listen for a dividend trigger it again, right after it fired.
fn retrigger_dividends(
    mut triggered: MessageReader<Triggered>,
    portfolio: Res<Portfolio>,
    mut sequence: ResMut<Sequence>,
) {
    for step in triggered.read() {
        let Triggered::Dividend(slot) = *step else {
            continue;
        };
        for joker in &portfolio.jokers {
            for _ in 0..joker.dividend_retriggers() {
                sequence.push_next(Triggered::RepeatedDividend(slot));
            }
        }
    }
}
