use std::collections::VecDeque;

use bevy::prelude::*;

use super::{AppState, GameState, GlobalRng, Portfolio};
use crate::consts::{TRIGGER_MIN_SECS, TRIGGER_SECS, TRIGGER_SPEEDUP};
use crate::stocks::Market;

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TurnEnded>()
            .add_message::<Triggered>()
            .add_message::<CashGained>()
            .add_message::<SharesGained>()
            .init_resource::<Sequence>()
            .add_systems(Update, on_turn_end.run_if(in_state(AppState::Playing)))
            .add_systems(
                Update,
                run_sequence
                    .in_set(SequenceSet)
                    .before(on_turn_end)
                    .run_if(not(in_state(AppState::GameOver))),
            )
            .add_systems(OnExit(AppState::GameOver), reset_sequence);
    }
}

#[derive(Message)]
pub struct TurnEnded;

#[derive(Message, Clone, Copy)]
pub enum Triggered {
    Dividend(usize),
    RepeatedDividend(usize),
    Joker(usize),
}

/// Sent after a trigger paid out: cash it added to the portfolio.
#[derive(Message, Clone, Copy)]
pub struct CashGained {
    pub source: Triggered,
    pub amount: f64,
}

/// Sent after a joker added shares to the holding in `slot`.
#[derive(Message, Clone, Copy)]
pub struct SharesGained {
    pub slot: usize,
    pub amount: f64,
}

#[derive(Resource, Default)]
pub struct Sequence {
    queue: VecDeque<Triggered>,
    wait: f32,
    /// Triggers fired so far this turn, the pace picks up with every one.
    fired: u32,
}

impl Sequence {
    /// Whether triggers are still firing (or the last one is still showing).
    pub fn is_busy(&self) -> bool {
        !self.queue.is_empty() || self.wait > 0.0
    }

    /// Makes `step` the next trigger to fire.
    pub fn push_next(&mut self, step: Triggered) {
        self.queue.push_front(step);
    }
}

/// The system that fires the end-of-turn triggers; order against it to react to them.
#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SequenceSet;

fn on_turn_end(
    mut turns: MessageReader<TurnEnded>,
    mut state: ResMut<GameState>,
    mut portfolio: ResMut<Portfolio>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for _ in turns.read() {
        portfolio.cash += portfolio.cash * state.interest;
        if state.date.is_last_week_of_month() {
            if portfolio.cash < state.monthly_debt_euro {
                next_state.set(AppState::GameOver);
                break;
            }
            portfolio.cash -= state.monthly_debt_euro;
            next_state.set(AppState::JokerShop);
        }
        state.date.advance_week();
    }
}

fn run_sequence(
    time: Res<Time>,
    mut turns: MessageReader<TurnEnded>,
    mut sequence: ResMut<Sequence>,
    mut triggered: MessageWriter<Triggered>,
    mut gains: MessageWriter<CashGained>,
    mut share_gains: MessageWriter<SharesGained>,
    mut portfolio: ResMut<Portfolio>,
    mut state: ResMut<GameState>,
    market: Res<Market>,
    mut rng: ResMut<GlobalRng>,
) {
    if turns.read().count() > 0 {
        let dividends = portfolio
            .slots
            .iter()
            .enumerate()
            .filter_map(|(slot, holding)| {
                let stock = market.get(&holding.as_ref()?.symbol)?;
                (stock.dividend_yield > 0.0).then_some(Triggered::Dividend(slot))
            });
        // Only jokers with something to do on turn end take a step (and flash).
        let jokers = (portfolio.jokers.iter().enumerate())
            .filter(|(_, joker)| joker.turn_effect().is_some())
            .map(|(slot, _)| Triggered::Joker(slot));
        sequence.queue = dividends.chain(jokers).collect();
        sequence.wait = 0.0;
        sequence.fired = 0;
    }
    sequence.wait -= time.delta_secs();
    if sequence.wait > 0.0 {
        return;
    }
    let Some(step) = sequence.queue.pop_front() else {
        return;
    };
    sequence.wait =
        (TRIGGER_SECS * TRIGGER_SPEEDUP.powi(sequence.fired as i32)).max(TRIGGER_MIN_SECS);
    sequence.fired += 1;
    let cash_before = portfolio.cash;
    let shares_before: Vec<f64> = (portfolio.slots.iter())
        .map(|h| h.as_ref().map_or(0.0, |h| h.shares))
        .collect();
    match step {
        Triggered::Dividend(slot) | Triggered::RepeatedDividend(slot) => {
            let Some(holding) = &portfolio.slots[slot] else {
                return;
            };
            let Some(stock) = market.get(&holding.symbol) else {
                return;
            };
            portfolio.cash += holding.shares * stock.dividend_per_share() as f64;
        }
        Triggered::Joker(slot) => {
            let Some(joker) = portfolio.jokers.get(slot).copied() else {
                return;
            };
            let Some(effect) = joker.turn_effect() else {
                return;
            };
            effect(&mut portfolio, &mut state, &mut rng);
        }
    }
    triggered.write(step);
    for (slot, before) in shares_before.into_iter().enumerate() {
        let after = portfolio.slots[slot].as_ref().map_or(0.0, |h| h.shares);
        if after > before {
            share_gains.write(SharesGained {
                slot,
                amount: after - before,
            });
        }
    }
    let amount = portfolio.cash - cash_before;
    if amount > 0.0 {
        gains.write(CashGained {
            source: step,
            amount,
        });
    }
}

fn reset_sequence(mut sequence: ResMut<Sequence>) {
    *sequence = Sequence::default();
}
