use bevy::prelude::*;

use crate::stocks::Ticker;

pub struct GameEventsPlugin;

impl Plugin for GameEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<GameEvent>();
    }
}

#[derive(Message, Clone)]
pub enum GameEvent {
    TurnEnded,
    Dividend(usize),
    Triggered(Triggered),
    CashGained(CashGained),
    SharesGained(SharesGained),
    Sold(Sold),
}

impl GameEvent {
    /// The holding slot of a `Dividend` event; only call it on one.
    pub fn div_slot(&self) -> usize {
        match self {
            GameEvent::Dividend(slot) => *slot,
            _ => unreachable!("not a Dividend event"),
        }
    }

    /// The ticker of a `Sold` event; only call it on one.
    pub fn sold_ticker(&self) -> Ticker {
        match self {
            GameEvent::Sold(Sold(ticker)) => *ticker,
            _ => unreachable!("not a Sold event"),
        }
    }
}

pub fn turns_ended(events: &mut MessageReader<GameEvent>) -> usize {
    events
        .read()
        .filter(|event| matches!(event, GameEvent::TurnEnded))
        .count()
}

#[derive(Clone, Copy)]
pub enum Triggered {
    Dividend(usize),
    /// One joker's boost to the dividend of the holding in `slot`.
    DividendBoost {
        slot: usize,
        joker: usize,
        boost: Boost,
    },
    Joker(usize),
}

/// How a joker raises a dividend: linear adds a flat amount, multiplicative scales it.
#[derive(Clone, Copy)]
pub enum Boost {
    Add(f64),
    Mul(f64),
}

/// Sent after a trigger paid out: cash it added to the portfolio.
#[derive(Clone, Copy)]
pub struct CashGained {
    pub source: Triggered,
    pub amount: f64,
}

/// Sent after a joker added shares to the holding in `slot`.
#[derive(Clone, Copy)]
pub struct SharesGained {
    pub slot: usize,
    pub amount: f64,
}

/// Sent when the player sells a share of the stock with this ticker.
#[derive(Clone)]
pub struct Sold(pub Ticker);
