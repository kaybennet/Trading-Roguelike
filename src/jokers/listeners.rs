use bevy::prelude::*;

use super::Joker;
use crate::general::{AppState, Boost, GameEvent, Portfolio, Sequence};
use crate::stocks::Market;

pub struct ListenersPlugin;

impl Plugin for ListenersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            joker_listeners
                .run_if(not(in_state(AppState::JokerShop)))
                .run_if(not(in_state(AppState::GameOver))),
        );
    }
}

impl Joker {
    pub fn listens_for(self, event: &GameEvent) -> bool {
        use GameEvent::*;
        use Joker::*;
        match self {
            MagicSell => matches!(event, Sold(_)),
            DoubleDividend => matches!(event, Dividend(_)),
            TripleDividend => matches!(event, Dividend(_)),
            QuadrupleDividend => matches!(event, Dividend(_)),
            IncreasedDividend => matches!(event, Dividend(_)),
            _ => false,
        }
    }

    pub fn trigger(self, slot: usize, e: &GameEvent, market: &mut Market, seq: &mut Sequence) {
        use Boost::*;
        use Joker::*;
        match self {
            MagicSell => market.get_mut(e.sold_ticker()).adj_price(Mul(1.01)),
            IncreasedDividend => seq.boost_div(e.div_slot(), slot, Add(1.0)),
            DoubleDividend => seq.boost_div(e.div_slot(), slot, Mul(2.0)),
            TripleDividend => seq.boost_div(e.div_slot(), slot, Mul(3.0)),
            QuadrupleDividend => seq.boost_div(e.div_slot(), slot, Mul(4.0)),
            _ => {}
        }
    }
}

fn joker_listeners(
    mut events: MessageReader<GameEvent>,
    portfolio: Res<Portfolio>,
    mut sequence: ResMut<Sequence>,
    mut market: ResMut<Market>,
) {
    for event in events.read() {
        for (slot, joker) in portfolio.jokers.iter().enumerate() {
            if joker.listens_for(event) {
                joker.trigger(slot, event, &mut market, &mut sequence);
            }
        }
    }
}
