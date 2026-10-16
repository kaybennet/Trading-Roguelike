use bevy::prelude::*;
use rand::RngExt;

use super::{Market, Ticker};
use crate::consts::OFFER_SIZE;
use crate::general::{AppState, GlobalRng};

pub struct OfferPlugin;

impl Plugin for OfferPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WeeklyOffer>()
            .add_systems(Update, fill_empty_offer)
            .add_systems(OnEnter(AppState::EndOfWeek), roll_offer)
            .add_systems(OnExit(AppState::GameOver), clear_offer);
    }
}

#[derive(Resource, Default)]
pub struct WeeklyOffer(pub Vec<Ticker>);

fn clear_offer(mut offer: ResMut<WeeklyOffer>) {
    offer.0.clear();
}

/// Covers a fresh run, where there is no offer yet.
fn fill_empty_offer(market: Res<Market>, rng: ResMut<GlobalRng>, offer: ResMut<WeeklyOffer>) {
    if offer.0.is_empty() {
        roll_offer(market, rng, offer);
    }
}

fn roll_offer(market: Res<Market>, mut rng: ResMut<GlobalRng>, mut offer: ResMut<WeeklyOffer>) {
    let pool: Vec<Ticker> = market.stocks.iter().map(|s| s.ticker).collect();
    if pool.len() >= OFFER_SIZE {
        offer.0 = pick_distinct(pool, OFFER_SIZE, &mut rng);
    }
}

fn pick_distinct(mut pool: Vec<Ticker>, count: usize, rng: &mut GlobalRng) -> Vec<Ticker> {
    (0..count).map(|_| pool.swap_remove(rng.random_range(0..pool.len()))).collect()
}
