use bevy::prelude::*;
use rand::RngExt;

use super::Market;
use crate::consts::OFFER_SIZE;
use crate::general::{AppState, GlobalRng, TurnEnded};

pub struct OfferPlugin;

impl Plugin for OfferPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WeeklyOffer>()
            .add_systems(Update, roll_offer)
            .add_systems(OnExit(AppState::GameOver), clear_offer);
    }
}

#[derive(Resource, Default)]
pub struct WeeklyOffer(pub Vec<String>);

fn clear_offer(mut offer: ResMut<WeeklyOffer>) {
    offer.0.clear();
}

fn roll_offer(
    mut turns: MessageReader<TurnEnded>,
    market: Res<Market>,
    mut rng: ResMut<GlobalRng>,
    mut offer: ResMut<WeeklyOffer>,
) {
    let turn_ended = turns.read().count() > 0;
    if !turn_ended && !offer.0.is_empty() {
        return;
    }
    let pool: Vec<String> = market.stocks.iter().map(|s| s.symbol.clone()).collect();
    if pool.len() >= OFFER_SIZE {
        offer.0 = pick_distinct(pool, OFFER_SIZE, &mut rng);
    }
}

fn pick_distinct(mut pool: Vec<String>, count: usize, rng: &mut GlobalRng) -> Vec<String> {
    (0..count)
        .map(|_| pool.swap_remove(rng.random_range(0..pool.len())))
        .collect()
}
