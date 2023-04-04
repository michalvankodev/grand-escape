use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::{loading::TextureAssets, environment::MAP_WIDTH, menu::MainCamera, GameState};

pub struct EnemyPlugin;

#[derive(Resource)]
pub struct EnemySpawnTimers {
    timers: Vec<Timer>,
}

impl Default for EnemySpawnTimers {
    fn default() -> Self {
        EnemySpawnTimers {
            timers: vec![Timer::new(Duration::from_secs(5), TimerMode::Repeating)],
        }
    }
}

#[derive(Component)]
pub struct Enemy;

impl Default for Enemy {
    fn default() -> Self {
        Enemy
    }
}

enum SpawnPosition {
    Left,
    Right,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimers>()
            .add_system(spawn_enemies.in_set(OnUpdate(GameState::Playing)));
    }
}

// TODO We will spawn enemies according to the timer slightly randomly
// Enemies shoot at us
// We shoot enemies
// Enemies will be better as the time goes on
// They will have health

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timers: ResMut<EnemySpawnTimers>,
    textures: Res<TextureAssets>,
    camera_query: Query<&Transform, With<MainCamera>>
) {
    let camera_position = camera_query.get_single().unwrap().translation.y;
    let next_spawn_position = camera_position + 600.;
    for timer in &mut spawn_timers.timers {
        timer.tick(time.delta());
        if timer.finished() {
            let position = get_random_spawn_position();
            let x = if let SpawnPosition::Right = position { MAP_WIDTH } else { 0.0 };
            commands
                .spawn(SpriteBundle {
                    texture: textures.enemy_cannon.clone(),
                    transform: Transform::from_translation(Vec3::new(x, next_spawn_position, 2.)),
                    // transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.)),
                    ..Default::default()
                })
                .insert(Enemy {
                    ..Default::default()
                });
        }
    }
}

fn get_random_spawn_position() -> SpawnPosition {
    let mut rng = rand::thread_rng();
    if rng.gen::<bool>() {
        SpawnPosition::Left
    } else {
        SpawnPosition::Right
    } 
}
