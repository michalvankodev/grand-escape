use bevy::{prelude::*, time::Stopwatch};

use crate::GameState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameTime>()
            .add_system(update_timer.in_set(OnUpdate(GameState::Playing)))
            .add_system(restart_score.in_schedule(OnEnter(GameState::Menu)));
    }
}

#[derive(Resource)]
pub struct GameTime {
    pub elapsed_time: Stopwatch,
}

impl Default for GameTime {
    fn default() -> Self {
        GameTime {
            elapsed_time: Stopwatch::new(),
        }
    }
}

fn update_timer(mut game_time: ResMut<GameTime>, time: Res<Time>) {
    game_time.elapsed_time.tick(time.delta());
}

fn restart_score() {}
