use bevy::prelude::*;

use crate::general::{Portfolio, Sequence, SequenceSet, Sold, Triggered};
use crate::stocks::Market;

pub struct ListenersPlugin;

impl Plugin for ListenersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, joker_listeners.after(SequenceSet));
    }
}

fn joker_listeners(
    mut triggered: MessageReader<Triggered>,
    mut sold: MessageReader<Sold>,
    portfolio: Res<Portfolio>,
    mut sequence: ResMut<Sequence>,
    mut market: ResMut<Market>,
) {
    for step in triggered.read() {
        let Triggered::Dividend(slot) = *step else {
            continue;
        };
        let times: usize = (portfolio.jokers.iter())
            .map(|joker| joker.dividend_multiplier())
            .product();
        for _ in 1..times {
            sequence.push_next(Triggered::RepeatedDividend(slot));
        }
    }

    for Sold(symbol) in sold.read() {
        let Some(stock) = market.stocks.iter_mut().find(|s| &s.symbol == symbol) else {
            continue;
        };
        for joker in &portfolio.jokers {
            stock.scale_price(joker.sell_price_factor());
        }
    }
}
