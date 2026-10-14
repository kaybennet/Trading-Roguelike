use bevy::prelude::Color;
use std::time::Duration;

use crate::general::{Date, GameState};

/// Seed of the global random number generator.
pub const SEED: u64 = 42;

// Debug overlay
pub const DEBUG_UPDATE_INTERVAL: Duration = Duration::from_millis(50);
pub const DEBUG_FONT_SIZE: f32 = 16.0;
pub const DEBUG_MARGIN: f32 = 4.0;
pub const DEBUG_BG_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

// Game screen
pub const SCORE_PANEL_WIDTH_PCT: f32 = 100.0 / 6.0;
pub const SQUARE_HEIGHT_PCT: f32 = 22.0;
pub const PANEL_GAP: f32 = 12.0;
pub const SCORE_FONT_SIZE: f32 = 18.0;
pub const SCORE_BOX_COLOR: Color = Color::srgb(0.15, 0.17, 0.22);
pub const PANEL_COLOR: Color = Color::srgb(0.25, 0.3, 0.4);
pub const START_GAME_STATE: GameState = GameState {
    monthly_debt_euro: 10.,
    invested_euro: 0.,
    date: Date {
        week: 0,
        month: 0,
        year: 0,
    },
    interest: 0.02,
};
/// Cash at the start of a run.
pub const START_CASH: f64 = 100.;
/// How many different stocks can be held at the start of a run.
pub const START_PORTFOLIO_SIZE: usize = 2;
pub const CIRCLE_SIZE: f32 = 120.0;
pub const CIRCLE_FONT_SIZE: f32 = 16.0;

// Stocks
pub const STOCK_MIN_PRICE: f32 = 1.0;
/// Four years of weekly prices are kept per stock.
pub const MARKET_HISTORY_WEEKS: usize = 4 * 12 * Date::WEEKS_PER_MONTH as usize;
pub const STOCK_FONT_SIZE: f32 = 16.0;
pub const STOCK_UP_COLOR: Color = Color::srgb(0.4, 0.9, 0.5);
pub const STOCK_DOWN_COLOR: Color = Color::srgb(0.95, 0.4, 0.4);
pub const TOP_BAR_HEIGHT_PCT: f32 = 10.0;
pub const TOP_BAR_FONT_SIZE: f32 = 24.0;
pub const BUTTON_COLOR: Color = Color::srgb(0.2, 0.5, 0.35);
pub const BUTTON_HOVER_COLOR: Color = Color::srgb(0.25, 0.6, 0.42);
pub const BUTTON_PRESSED_COLOR: Color = Color::srgb(0.15, 0.4, 0.28);
pub const SQUARE_BUTTON_FONT_SIZE: f32 = 14.0;

// Trading
pub const OFFER_SIZE: usize = 3;
pub const SELL_FRACTIONS: [(&str, f64); 3] = [("10%", 0.1), ("50%", 0.5), ("All", 1.0)];
pub const TICKER_TOP_PCT: f32 = 28.0;
pub const TICKER_HEIGHT_PCT: f32 = 32.0;
pub const TICKER_FONT_SIZE: f32 = 36.0;
pub const TICKER_CARD_WIDTH_PCT: f32 = 28.0;
pub const MIN_SHARES: f64 = 0.001;

pub const GAME_OVER_BG_COLOR: Color = Color::srgb(0.1, 0.02, 0.02);
pub const GAME_OVER_FONT_SIZE: f32 = 40.0;

// Jokers
pub const JOKER_COUNT: u32 = 5;
pub const JOKER_PANEL_WIDTH_PCT: f32 = 20.0;
/// Height of one joker slot as a percent of the screen height.
pub const JOKER_HEIGHT_PCT: f32 = 8.0;
pub const JOKER_FONT_SIZE: f32 = 20.0;
