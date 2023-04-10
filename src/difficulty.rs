use bevy::prelude::*;

use crate::{score::GameScore, GameState};

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Difficulty {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Initial,
    Medium,
    Hard,
}

pub struct DifficultyPlugin;

impl Plugin for DifficultyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(change_difficulty.in_set(OnUpdate(GameState::Playing)));
    }
}

fn change_difficulty(
    game_score: Res<GameScore>,
    state: Res<State<Difficulty>>,
    mut next_state: ResMut<NextState<Difficulty>>,
) {
    let score = game_score.score + game_score.distance_traveled as i32 / 50;
    match state.0 {
        Difficulty::Initial => {
            // if score > 200 {
            if score > 10 {
                next_state.set(Difficulty::Medium);
            }
        }
        Difficulty::Medium => {
            if score > 400 {
                next_state.set(Difficulty::Hard);
            }
        }
        _ => {}
    }
}
