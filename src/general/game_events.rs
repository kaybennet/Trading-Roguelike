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
    EndOfTurn(EndOfWeekEvent),
    CashGained(CashGained),
    SharesGained(SharesGained),
    Sold(Sold),
}

#[derive(Clone, Copy)]
pub enum EndOfWeekEvent {
    Dividend(usize),
    DividendBoost { slot: usize, joker: usize, boost: Boost },
    Joker(usize),
}

#[derive(Clone, Copy)]
pub enum Boost {
    Add(f64),
    Mul(f64),
}

#[derive(Clone, Copy)]
pub struct CashGained {
    pub source: EndOfWeekEvent,
    pub amount: f64,
}

#[derive(Clone, Copy)]
pub struct SharesGained {
    pub slot: usize,
    pub amount: f64,
}

#[derive(Clone)]
pub struct Sold(pub Ticker);
