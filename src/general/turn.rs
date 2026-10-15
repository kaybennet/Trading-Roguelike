use std::collections::VecDeque;

use bevy::prelude::*;

use super::{
    AppState, Boost, CashGained, GameEvent, GameState, GlobalRng, Portfolio, SharesGained,
    Triggered, turns_ended,
};
use crate::consts::{TRIGGER_MIN_SECS, TRIGGER_SECS, TRIGGER_SPEEDUP};
use crate::stocks::Market;

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Sequence>()
            .add_systems(
                Update,
                (apply_stocks, apply_jokers, advance_time, open_shop)
                    .chain()
                    .run_if(in_state(AppState::EndOfWeek)),
            )
            .add_systems(OnExit(AppState::GameOver), reset_sequence);
    }
}

#[derive(Resource, Default)]
pub struct Sequence {
    queue: VecDeque<Triggered>,
    wait: f32,
    fired: u32,
    open_shop: bool,
    /// What the dividend being boosted has paid out so far.
    dividend: f64,
}

impl Sequence {
    pub fn is_busy(&self) -> bool {
        !self.queue.is_empty() || self.wait > 0.0
    }

    pub fn push_next(&mut self, step: Triggered) {
        self.queue.push_front(step);
    }

    /// Queues a boost to the dividend of `holding`, after the boosts already queued, so
    /// the jokers apply in portfolio order: +€1 then x2 is worth more than x2 then +€1.
    pub fn boost_div(&mut self, holding: usize, joker: usize, boost: Boost) {
        let queued = (self.queue.iter())
            .take_while(|step| matches!(step, Triggered::DividendBoost { .. }))
            .count();
        let step = Triggered::DividendBoost {
            slot: holding,
            joker,
            boost,
        };
        self.queue.insert(queued, step);
    }
}

/// Turn end, step 1: queues a trigger for every dividend-paying holding.
fn apply_stocks(
    mut events: MessageReader<GameEvent>,
    mut sequence: ResMut<Sequence>,
    portfolio: Res<Portfolio>,
    market: Res<Market>,
) {
    if turns_ended(&mut events) == 0 {
        return;
    }
    sequence.queue = (portfolio.slots.iter().enumerate())
        .filter_map(|(slot, holding)| {
            let stock = market.get(holding.as_ref()?.ticker);
            (stock.dividend_yield > 0.0).then_some(Triggered::Dividend(slot))
        })
        .collect();
    sequence.wait = 0.0;
    sequence.fired = 0;
}

/// Turn end, step 2: queues a trigger for every joker with something to do on turn end.
fn apply_jokers(
    mut events: MessageReader<GameEvent>,
    mut sequence: ResMut<Sequence>,
    portfolio: Res<Portfolio>,
) {
    if turns_ended(&mut events) == 0 {
        return;
    }
    let jokers = (portfolio.jokers.iter().enumerate())
        .filter(|(_, joker)| joker.turn_effect().is_some())
        .map(|(slot, _)| Triggered::Joker(slot));
    sequence.queue.extend(jokers);
}

/// Turn end, step 3: fires the next queued trigger once its wait is up, then books the
/// turn itself: interest, the monthly debt, and the next week.
fn advance_time(
    time: Res<Time>,
    // Reading and writing the same message type in one system needs a `ParamSet`.
    mut events: ParamSet<(MessageReader<GameEvent>, MessageWriter<GameEvent>)>,
    mut sequence: ResMut<Sequence>,
    mut portfolio: ResMut<Portfolio>,
    mut state: ResMut<GameState>,
    market: Res<Market>,
    mut rng: ResMut<GlobalRng>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let turns = turns_ended(&mut events.p0());
    sequence.wait -= time.delta_secs();
    if sequence.wait <= 0.0 {
        if let Some(step) = sequence.queue.pop_front() {
            sequence.wait =
                (TRIGGER_SECS * TRIGGER_SPEEDUP.powi(sequence.fired as i32)).max(TRIGGER_MIN_SECS);
            sequence.fired += 1;
            fire(
                step,
                &mut portfolio,
                &mut state,
                &market,
                &mut rng,
                &mut sequence.dividend,
                &mut events.p1(),
            );
        }
    }
    for _ in 0..turns {
        portfolio.cash += portfolio.cash * state.interest;
        if state.date.is_last_week_of_month() {
            if portfolio.cash < state.monthly_debt_euro {
                next_state.set(AppState::GameOver);
                break;
            }
            portfolio.cash -= state.monthly_debt_euro;
            sequence.open_shop = true;
        }
        state.date.advance_week();
    }
}

/// Once every trigger has played out, leaves the week: to the shop at month end, else on
/// to the next week.
fn open_shop(mut sequence: ResMut<Sequence>, mut next_state: ResMut<NextState<AppState>>) {
    // `advance_time` may already have queued the game over.
    if sequence.is_busy() || matches!(*next_state, NextState::Pending(_)) {
        return;
    }
    next_state.set(if sequence.open_shop {
        AppState::JokerShop
    } else {
        AppState::DuringWeek
    });
    sequence.open_shop = false;
}

fn fire(
    step: Triggered,
    portfolio: &mut Portfolio,
    state: &mut GameState,
    market: &Market,
    rng: &mut GlobalRng,
    div: &mut f64,
    out: &mut MessageWriter<GameEvent>,
) {
    let cash_before = portfolio.cash;
    let shares_before: Vec<f64> = (portfolio.slots.iter())
        .map(|h| h.as_ref().map_or(0.0, |h| h.shares))
        .collect();
    match step {
        Triggered::Dividend(slot) => {
            let Some(holding) = &portfolio.slots[slot] else {
                return;
            };
            *div = holding.shares * market.get(holding.ticker).div_per_share() as f64;
            portfolio.cash += *div;
        }
        Triggered::DividendBoost { boost, .. } => {
            let extra = match boost {
                Boost::Add(amount) => amount,
                Boost::Mul(factor) => *div * (factor - 1.0),
            };
            *div += extra;
            portfolio.cash += extra;
        }
        Triggered::Joker(slot) => {
            let Some(joker) = portfolio.jokers.get(slot).copied() else {
                return;
            };
            let Some(effect) = joker.turn_effect() else {
                return;
            };
            effect(portfolio, state, rng);
        }
    }
    out.write(GameEvent::Triggered(step));
    if let Triggered::Dividend(slot) = step {
        out.write(GameEvent::Dividend(slot));
    }
    for (slot, before) in shares_before.into_iter().enumerate() {
        let after = portfolio.slots[slot].as_ref().map_or(0.0, |h| h.shares);
        if after > before {
            out.write(GameEvent::SharesGained(SharesGained {
                slot,
                amount: after - before,
            }));
        }
    }
    let amount = portfolio.cash - cash_before;
    if amount > 0.0 {
        out.write(GameEvent::CashGained(CashGained {
            source: step,
            amount,
        }));
    }
}

fn reset_sequence(mut sequence: ResMut<Sequence>) {
    *sequence = Sequence::default();
}
