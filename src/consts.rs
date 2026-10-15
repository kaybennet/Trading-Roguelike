use std::time::Duration;

use crate::general::{Date, GameState};

/// Seed of the global random number generator.
pub const SEED: u64 = 42;

// Debug overlay
pub const DEBUG_UPDATE_INTERVAL: Duration = Duration::from_millis(50);
pub const DEBUG_FONT_SIZE: f32 = 16.0;
pub const DEBUG_MARGIN: f32 = 4.0;

// Game screen
pub const SCORE_PANEL_WIDTH_PCT: f32 = 100.0 / 6.0;
pub const SQUARE_HEIGHT_PCT: f32 = 22.0;
pub const PANEL_GAP: f32 = 12.0;
pub const SCORE_FONT_SIZE: f32 = 18.0;

// Theme
/// The layout is designed for this size; the UI zooms so the window never gets smaller.
pub const DESIGN_WIDTH: f32 = 1280.0;
pub const DESIGN_HEIGHT: f32 = 720.0;
pub const CORNER_RADIUS: u8 = 10;
/// How long hover animations take to fade in and out.
pub const HOVER_SECS: f32 = 0.12;
/// How far a hovered card rises.
pub const HOVER_LIFT: f32 = 3.0;
pub const START_GAME_STATE: GameState = GameState {
    monthly_debt_euro: 10.,
    invested_euro: 0.,
    date: Date {
        week: 0,
        month: 0,
        year: 0,
    },
    interest: 0.02,
    modifiers: Vec::new(),
};
/// Cash at the start of a run.
pub const START_CASH: f64 = 100.;
/// How many different stocks can be held at the start of a run.
pub const START_PORTFOLIO_SIZE: usize = 2;
/// Fraction of a joker's price paid out when it is sold.
pub const JOKER_SELL_FRACTION: f64 = 0.5;

// Stocks
pub const STOCK_MIN_PRICE: f32 = 1.0;
/// Four years of weekly prices are kept per stock.
pub const MARKET_HISTORY_WEEKS: usize = 4 * 12 * Date::WEEKS_PER_MONTH as usize;
pub const STOCK_FONT_SIZE: f32 = 16.0;
pub const TOP_BAR_HEIGHT_PCT: f32 = 10.0;
pub const TOP_BAR_FONT_SIZE: f32 = 24.0;

// Trading
/// Holding a ticker card buys (or a stock square sells) a share after this long, then
/// repeats. The repeat interval starts at `HOLD_REPEAT_START_SECS` and shrinks to
/// `HOLD_REPEAT_MIN_SECS` over `HOLD_ACCEL_SECS` of continued holding.
pub const HOLD_DELAY_SECS: f64 = 0.5;
pub const HOLD_REPEAT_START_SECS: f64 = 0.2;
pub const HOLD_REPEAT_MIN_SECS: f64 = 0.02;
pub const HOLD_ACCEL_SECS: f64 = 3.0;
pub const OFFER_SIZE: usize = 3;
pub const TICKER_TOP_PCT: f32 = 28.0;
pub const TICKER_HEIGHT_PCT: f32 = 32.0;
pub const TICKER_FONT_SIZE: f32 = 36.0;
pub const TICKER_CARD_WIDTH_PCT: f32 = 28.0;
pub const MIN_SHARES: f64 = 0.001;

pub const GAME_OVER_FONT_SIZE: f32 = 40.0;

// Jokers
pub const JOKER_COUNT: u32 = 5;
pub const JOKER_PANEL_WIDTH_PCT: f32 = 20.0;
/// Height of one joker slot as a percent of the screen height.
pub const JOKER_HEIGHT_PCT: f32 = 8.0;
pub const JOKER_FONT_SIZE: f32 = 20.0;
/// Pause between one end-of-turn trigger (dividend or joker) and the next.
pub const TRIGGER_SECS: f32 = 0.6;
/// Each trigger in a sequence waits this fraction of the previous pause, down to a floor.
pub const TRIGGER_SPEEDUP: f32 = 0.8;
pub const TRIGGER_MIN_SECS: f32 = 0.04;
/// How long a trigger's flash takes to fade out.
pub const TRIGGER_FLASH_SECS: f32 = 0.2;
/// How long the "+€x" that floats above a trigger lasts, and how far it rises.
pub const FLOAT_SECS: f32 = 1.0;
pub const FLOAT_RISE: f32 = 40.0;
pub const FLOAT_FONT_SIZE: f32 = 22.0;
/// How many cards the shop offers after each paid debt.
pub const SHOP_OFFER_SIZE: usize = 4;

// Modifiers
