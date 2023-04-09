use bevy::prelude::*;
use rand::Rng;
use std::{f32::consts::PI, time::Duration};

use crate::{
    environment::{Collidable, MAP_WIDTH},
    health::Health,
    loading::TextureAssets,
    menu::MainCamera,
    power_up::PowerUp,
    GameState,
};

#[derive(Component)]
pub struct ObstacleTile;

#[derive(Resource)]
pub struct ObstacleSpawnTimers {
    timers: Vec<Timer>,
}

const OBSTACLE_SIZES: [bevy::prelude::Vec2; 6] = [
    Vec2::new(55., 57.),
    Vec2::new(48., 50.),
    Vec2::new(55., 37.),
    Vec2::new(32., 11.),
    Vec2::new(32., 9.),
    Vec2::new(32., 9.),
];

impl Default for ObstacleSpawnTimers {
    fn default() -> Self {
        ObstacleSpawnTimers {
            timers: vec![
                Timer::new(Duration::from_secs(2), TimerMode::Repeating),
                Timer::new(Duration::from_secs(4), TimerMode::Repeating),
            ],
        }
    }
}

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ObstacleSpawnTimers>()
            .add_system(despawn_obstacles.in_schedule(OnEnter(GameState::Restart)))
            .add_system(detect_dead_obstacles.in_set(OnUpdate(GameState::Playing)))
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
                textures.obstacle_wood1.clone(),
                textures.obstacle_wood2.clone(),
                textures.obstacle_wood3.clone(),
            ];
            let mut rng = rand::thread_rng();
            let which_one_index = rng.gen_range(0..6);
            let random_angle = rng.gen_range(0.0..2. * PI);
            let obstacle_texture = &available_obstacles[which_one_index];
            let size = OBSTACLE_SIZES[which_one_index];
            let health = if which_one_index > 2 { 1 } else { 100 };
            let damage = if which_one_index > 2 { 1 } else { 2 };
            let immune = which_one_index > 2;
            let position = get_random_obstacle_spawn_position();
            commands
                .spawn(SpriteBundle {
                    texture: obstacle_texture.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        position,
                        next_spawn_position,
                        2.,
                    ))
                    .with_rotation(Quat::from_rotation_z(random_angle)),
                    // transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.)),
                    ..Default::default()
                })
                .insert(ObstacleTile)
                .insert(Health {
                    max_health: health,
                    health_amount: health,
                    size,
                    immune_to_bullets: immune,
                })
                .insert(Collidable {
                    size,
                    damage,
                    is_alive: true,
                });
            let mut rng = rand::thread_rng();
            let duration = rng.gen_range(2500..5000); // TODO change with increasing difficulty
            timer.set_duration(Duration::from_millis(duration));
        }
    }
}

pub fn get_random_obstacle_spawn_position() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(30.0..MAP_WIDTH - 30.)
}

fn detect_dead_obstacles(
    mut obstacles_q: Query<(&mut Collidable, &mut Handle<Image>, &Health), Without<PowerUp>>,
    textures: Res<TextureAssets>,
) {
    for (mut obstacle, mut handle, health) in obstacles_q.iter_mut() {
        if obstacle.is_alive == false {
            continue;
        }
        if health.health_amount <= 0 {
            obstacle.is_alive = false;
            *handle = textures.obstacle_wood_dead.clone();
        }
    }
}

fn despawn_obstacles(
    mut commands: Commands,
    obstacle_q: Query<Entity, With<ObstacleTile>>,
    mut obstacle_spawn_timers: ResMut<ObstacleSpawnTimers>,
) {
    for entity in obstacle_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
    *obstacle_spawn_timers = ObstacleSpawnTimers::default();
}
