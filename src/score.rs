use bevy::{prelude::*, time::Stopwatch};

use crate::{player::Player, GameState, environment::MAP_HEIGHT};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameScore>()
            .add_system(update_timer.in_set(OnUpdate(GameState::Playing)))
            .add_system(update_distance.in_set(OnUpdate(GameState::Playing)))
            .add_system(restart_score.in_schedule(OnEnter(GameState::Restart)));
    }
}

#[derive(Resource)]
pub struct GameScore {
    pub elapsed_time: Stopwatch,
    pub score: i32,
    pub distance_traveled: f32,
}

impl Default for GameScore {
    fn default() -> Self {
        GameScore {
            elapsed_time: Stopwatch::new(),
            score: 0,
            distance_traveled: 0.,
        }
    }
}

fn update_timer(mut game_score: ResMut<GameScore>, time: Res<Time>) {
    game_score.elapsed_time.tick(time.delta());
}

fn update_distance(mut game_score: ResMut<GameScore>, player_q: Query<&Transform, With<Player>>) {
    let start_y = MAP_HEIGHT / 2.;
    let distance_in_world = player_q.get_single().unwrap().translation.y - start_y ;
    game_score.distance_traveled = distance_in_world / 16.;
}

// TODO Pause , pause menu / spawn despawn
// TODO Restart, spawn / despawn correctly uis and enemies
// TODO SOUND
// TODO Heath regenaration / Picking up upgrades from barrels
// TODO Weapon upgrades
// TODO Swap cannon / munition textures
// TODO Side effects
// TODO Pirates - Standing on water / more health / more power

fn restart_score(mut game_score: ResMut<GameScore>) {
    *game_score = GameScore::default();
}
