use bevy::prelude::*;

use super::AppState;
use crate::consts::START_GAME_STATE;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .add_systems(OnExit(AppState::GameOver), reset_game_state);
    }
}

#[derive(Resource)]
pub struct GameState {
    pub monthly_debt_euro: f64,
    pub invested_euro: f64,
    pub date: Date,
    pub interest: f64,
}

#[derive(Default)]
pub struct Date {
    pub week: u32,
    pub month: usize,
    pub year: u32,
}

impl Date {
    pub const WEEKS_PER_MONTH: u32 = 4;
    pub const MONTHS: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    pub fn month_name(&self) -> &str {
        Self::MONTHS[self.month]
    }

    pub fn is_last_week_of_month(&self) -> bool {
        self.week == Self::WEEKS_PER_MONTH - 1
    }

    pub fn advance_week(&mut self) {
        self.week += 1;
        if self.week == Self::WEEKS_PER_MONTH {
            self.week = 0;
            self.month = (self.month + 1) % Self::MONTHS.len();
            if self.month == 0 {
                self.year += 1;
            }
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        START_GAME_STATE
    }
}

fn reset_game_state(mut state: ResMut<GameState>) {
    *state = GameState::default();
}
