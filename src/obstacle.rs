use bevy::prelude::*;
use rand::Rng;
use std::{time::Duration, f32::consts::PI};

use crate::{GameState, loading::TextureAssets, menu::MainCamera, environment::{Collidable, MAP_WIDTH}, health::Health};

#[derive(Component)]
pub struct ObstacleTile;

#[derive(Resource)]
pub struct ObstacleSpawnTimers {
    timers: Vec<Timer>,
}

const OBSTACLE_SIZES: [bevy::prelude::Vec2; 3] = [
    Vec2::new(58., 59.),
    Vec2::new(51., 53.),
    Vec2::new(60., 41.),
];

impl Default for ObstacleSpawnTimers {
    fn default() -> Self {
        ObstacleSpawnTimers {
            timers: vec![Timer::new(Duration::from_secs(2), TimerMode::Repeating), Timer::new(Duration::from_secs(4), TimerMode::Repeating)],
        }
    }
}

// TODO Rotation of rocks random

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
            app.init_resource::<ObstacleSpawnTimers>()
            .add_system(spawn_obstacles.in_set(OnUpdate(GameState::Playing)));
    }
}

fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timers: ResMut<ObstacleSpawnTimers>,
    textures: Res<TextureAssets>,
    camera_query: Query<&Transform, With<MainCamera>>,
) {
    let camera_position = camera_query.get_single().unwrap().translation.y;
    let next_spawn_position = camera_position + 600.;
    for timer in &mut spawn_timers.timers {
        timer.tick(time.delta());
        if timer.finished() {
            let available_obstacles = [
                textures.obstacle_rock1.clone(),
                textures.obstacle_rock2.clone(),
                textures.obstacle_rock3.clone(),
            ];
            let mut rng = rand::thread_rng();
            let which_one_index = rng.gen_range(0..2);
            let random_angle = rng.gen_range(0.0..2. * PI);
            let obstacle_texture = &available_obstacles[which_one_index];
            let size = OBSTACLE_SIZES[which_one_index];
            let position = get_random_obstacle_spawn_position();
            commands
                .spawn(SpriteBundle {
                    texture: obstacle_texture.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        position,
                        next_spawn_position,
                        2.,
                    )).with_rotation(Quat::from_rotation_z(random_angle)),
                    // transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.)),
                    ..Default::default()
                })
                .insert(ObstacleTile)
                .insert(Health { max_health: 100, health_amount: 100, size })
                .insert(Collidable {
                    size
                });
            let mut rng = rand::thread_rng();
            let duration = rng.gen_range(2500..5000); // TODO change with increasing difficulty
            timer.set_duration(Duration::from_millis(duration));
        }
    }
}

fn get_random_obstacle_spawn_position() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(30.0..MAP_WIDTH - 30.)
}
