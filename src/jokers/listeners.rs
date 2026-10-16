use bevy::prelude::*;

use super::{Joker, JokerKind};
use crate::general::{AppState, Boost, EndOfWeekEvent, GameEvent, Portfolio, Sequence, Sold};
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
    /// Reacts to `e` if this joker listens for it; every other event is ignored.
    pub fn trigger(self, e: &GameEvent, market: &mut Market, seq: &mut Sequence) {
        use Boost::*;
        use EndOfWeekEvent::Dividend;
        use GameEvent as G;
        use JokerKind::*;
        let slot = self.slot as usize;
        match (self.kind, e) {
            (MagicSell, G::Sold(Sold(ticker))) => market.get_mut(*ticker).adj_price(Mul(1.01)),
            (IncreasedDividend, G::EndOfTurn(Dividend(div))) => seq.boost_div(*div, slot, Add(1.0)),
            (DoubleDividend, G::EndOfTurn(Dividend(div))) => seq.boost_div(*div, slot, Mul(2.0)),
            (TripleDividend, G::EndOfTurn(Dividend(div))) => seq.boost_div(*div, slot, Mul(3.0)),
            (QuadrupleDividend, G::EndOfTurn(Dividend(div))) => seq.boost_div(*div, slot, Mul(4.0)),
            _ => {}
        }
    }
}

fn joker_listeners(
    mut events: MessageReader<GameEvent>,
    p: Res<Portfolio>,
    mut seq: ResMut<Sequence>,
    mut market: ResMut<Market>,
) {
    for e in events.read() {
        for joker in &p.jokers {
            joker.trigger(e, &mut market, &mut seq);
        }
    }
}
