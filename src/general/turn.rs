use std::collections::VecDeque;

use bevy::prelude::*;

use super::{
    AppState, Boost, CashGained, EndOfWeekEvent, GameEvent, GameState, GlobalRng, Portfolio,
    SharesGained,
};
use crate::consts::{TRIGGER_MIN_SECS, TRIGGER_SECS, TRIGGER_SPEEDUP};
use crate::stocks::Market;

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Sequence>()
            .add_systems(
                OnEnter(AppState::EndOfWeek),
                (apply_stocks, apply_jokers, book_turn).chain(),
            )
            .add_systems(
                Update,
                (play_queue, open_shop).chain().run_if(in_state(AppState::EndOfWeek)),
            )
            .add_systems(OnExit(AppState::GameOver), reset_sequence);
    }
}

#[derive(Resource, Default)]
pub struct Sequence {
    queue: VecDeque<EndOfWeekEvent>,
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

    pub fn push_next(&mut self, step: EndOfWeekEvent) {
        self.queue.push_front(step);
    }

    pub fn boost_div(&mut self, holding: usize, joker: usize, boost: Boost) {
        let queued = (self.queue.iter())
            .take_while(|step| matches!(step, EndOfWeekEvent::DividendBoost { .. }))
            .count();
        let step = EndOfWeekEvent::DividendBoost { slot: holding, joker, boost };
        self.queue.insert(queued, step);
    }
}

fn apply_stocks(mut seq: ResMut<Sequence>, portfolio: Res<Portfolio>, market: Res<Market>) {
    let div_events: Vec<EndOfWeekEvent> = portfolio
        .holdings()
        .iter()
        .filter(|h| h.div_claim(&market) >= 0.)
        .map(|h| EndOfWeekEvent::Dividend(portfolio.slot_of(h.ticker).unwrap()))
        .collect();
    seq.queue.extend(div_events);
    seq.wait = 0.0;
    seq.fired = 0;
}

fn apply_jokers(mut sequence: ResMut<Sequence>, portfolio: Res<Portfolio>) {
    let jokers = (portfolio.jokers.iter().enumerate())
        .filter(|(_, joker)| joker.turn_effect().is_some())
        .map(|(slot, _)| EndOfWeekEvent::Joker(slot));
    sequence.queue.extend(jokers);
}

fn book_turn(
    mut seq: ResMut<Sequence>,
    mut portfolio: ResMut<Portfolio>,
    mut state: ResMut<GameState>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    portfolio.cash += portfolio.cash * state.interest;
    if state.date.is_last_week_of_month() {
        if portfolio.cash < state.monthly_debt_euro {
            next_state.set(AppState::GameOver);
            return;
        }
        portfolio.cash -= state.monthly_debt_euro;
        seq.open_shop = true;
    }
    state.date.advance_week();
}

fn play_queue(
    time: Res<Time>,
    mut events: MessageWriter<GameEvent>,
    mut sequence: ResMut<Sequence>,
    mut portfolio: ResMut<Portfolio>,
    mut state: ResMut<GameState>,
    market: Res<Market>,
    mut rng: ResMut<GlobalRng>,
) {
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
    fire(step, &mut portfolio, &mut state, &market, &mut rng, &mut sequence.dividend, &mut events);
}

fn open_shop(mut sequence: ResMut<Sequence>, mut next_state: ResMut<NextState<AppState>>) {
    if sequence.is_busy() || matches!(*next_state, NextState::Pending(_)) {
        return;
    }
    next_state.set(if sequence.open_shop { AppState::JokerShop } else { AppState::DuringWeek });
    sequence.open_shop = false;
}

fn fire(
    step: EndOfWeekEvent,
    portfolio: &mut Portfolio,
    state: &mut GameState,
    market: &Market,
    rng: &mut GlobalRng,
    div: &mut f64,
    out: &mut MessageWriter<GameEvent>,
) {
    let cash_before = portfolio.cash;
    let shares_before: Vec<f64> =
        (portfolio.slots.iter()).map(|h| h.as_ref().map_or(0.0, |h| h.shares)).collect();
    use EndOfWeekEvent as E;
    match step {
        E::Dividend(slot) => {
            let Some(holding) = &portfolio.slots[slot] else {
                return;
            };
            *div = holding.shares * market.get(holding.ticker).div_per_share() as f64;
            portfolio.cash += *div;
        }
        E::DividendBoost { boost, .. } => {
            let extra = match boost {
                Boost::Add(amount) => amount,
                Boost::Mul(factor) => *div * (factor - 1.0),
            };
            *div += extra;
            portfolio.cash += extra;
        }
        E::Joker(slot) => {
            let Some(joker) = portfolio.jokers.get(slot).copied() else {
                return;
            };
            let Some(effect) = joker.turn_effect() else {
                return;
            };
            effect(portfolio, state, rng);
        }
    }
    out.write(GameEvent::EndOfTurn(step));
    for (slot, before) in shares_before.into_iter().enumerate() {
        let after = portfolio.slots[slot].as_ref().map_or(0.0, |h| h.shares);
        if after > before {
            out.write(GameEvent::SharesGained(SharesGained { slot, amount: after - before }));
        }
    }
    let amount = portfolio.cash - cash_before;
    if amount > 0.0 {
        out.write(GameEvent::CashGained(CashGained { source: step, amount }));
    }
}

fn reset_sequence(mut sequence: ResMut<Sequence>) {
    *sequence = Sequence::default();
}
