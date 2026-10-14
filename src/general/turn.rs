use bevy::prelude::*;

use super::{AppState, GameState, Portfolio};

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TurnEnded>()
            .add_systems(Update, on_turn_end.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Message)]
pub struct TurnEnded;

fn on_turn_end(
    mut turns: MessageReader<TurnEnded>,
    mut state: ResMut<GameState>,
    mut portfolio: ResMut<Portfolio>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for _ in turns.read() {
        portfolio.cash += portfolio.cash * state.interest;
        // The debt is due at the end of the month's last week.
        if state.date.is_last_week_of_month() {
            if portfolio.cash < state.monthly_debt_euro {
                next_state.set(AppState::GameOver);
                break;
            }
            portfolio.cash -= state.monthly_debt_euro;
        }
        state.date.advance_week();
    }
}
