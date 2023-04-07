use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use rand::Rng;

use crate::{
    environment::{LAND_TILE_SIZE, MAP_WIDTH},
    health::{Bullet, Health},
    loading::TextureAssets,
    menu::MainCamera,
    player::{Movement, Player},
    GameState,
};

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
pub struct Enemy {
    vector: Vec2,
    shooting_timer: Timer,
    is_alive: bool,
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            vector: Vec2::new(0., 0.),
            shooting_timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
            is_alive: true,
        }
    }
}

enum SpawnPosition {
    Left,
    Right,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimers>()
            .add_system(spawn_enemies.in_set(OnUpdate(GameState::Playing)))
            .add_system(enemies_shoot_at_player.in_set(OnUpdate(GameState::Playing)))
            .add_system(despawn_enemies.in_set(OnUpdate(GameState::Playing)))
            .add_system(detect_killed_enemies.in_set(OnUpdate(GameState::Playing)))
            .add_system(enemies_face_player.in_set(OnUpdate(GameState::Playing)));
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
    camera_query: Query<&Transform, With<MainCamera>>,
) {
    let camera_position = camera_query.get_single().unwrap().translation.y;
    let next_spawn_position = camera_position + 600.;
    for timer in &mut spawn_timers.timers {
        timer.tick(time.delta());
        if timer.finished() {
            let position = get_random_spawn_position();
            let x = if let SpawnPosition::Right = position {
                MAP_WIDTH + LAND_TILE_SIZE 
            } else {
                0.0 - LAND_TILE_SIZE
            };
            commands
                .spawn(SpriteBundle {
                    texture: textures.enemy_cannon.clone(),
                    transform: Transform::from_translation(Vec3::new(x, next_spawn_position, 4.)),
                    // transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.)),
                    ..Default::default()
                })
                .insert(Enemy {
                    ..Default::default()
                })
                .insert(Health {
                    health_amount: 2,
                    size: Vec2::new(64., 64.),
                });
            let mut rng = rand::thread_rng();
            let duration = rng.gen_range(3500..5000);
            timer.set_duration(Duration::from_millis(duration));
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

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    camera_query: Query<&Transform, With<MainCamera>>,
) {
    let camera_translation = camera_query.get_single().unwrap().translation;
    for (enemy_entity, transform) in enemy_query.iter() {
        if transform.translation.y < camera_translation.y - 600. {
            commands.entity(enemy_entity).despawn();
        }
    }
}

fn enemies_face_player(
    mut transform_query: Query<(&mut Transform, &mut Enemy), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_translation = player_query.get_single().unwrap().translation;

    for (mut transform, mut enemy) in transform_query.iter_mut() {
        if enemy.is_alive == false {
            continue;
        }
        let mut vector = (player_translation - transform.translation).truncate();
        vector.y = vector.y + 100.;
        enemy.vector = vector.normalize();
        let angle = enemy.vector.y.atan2(enemy.vector.x) - PI / 2.0;
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

fn enemies_shoot_at_player(
    mut commands: Commands,
    mut shooters_query: Query<(&mut Enemy, &Transform, Entity)>,
    time: Res<Time>,
    textures: Res<TextureAssets>,
) {
    for (mut enemy, transform, enemy_entity) in shooters_query.iter_mut() {
        enemy.shooting_timer.tick(time.delta());
        if enemy.is_alive == false {
            continue;
        }
        if enemy.shooting_timer.finished() {
            let enemy_translation = transform.translation.truncate();
            commands
                .spawn(SpriteBundle {
                    texture: textures.bullet.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        enemy_translation.x,
                        enemy_translation.y,
                        3.,
                    )),
                    ..Default::default()
                })
                .insert(Bullet::new(enemy_entity))
                .insert(Movement {
                    vector: enemy.vector,
                    speed: 350.0,
                    ..Default::default()
                });
        }
    }
}

fn detect_killed_enemies(
    mut enemies_q: Query<(&mut Enemy, &mut Handle<Image>, &Health)>,
    textures: Res<TextureAssets>,
) {
    for (mut enemy, mut handle, health) in enemies_q.iter_mut() {
        if enemy.is_alive == false {
            continue;
        }
        if health.health_amount <= 0 {
            enemy.is_alive = false;
            *handle = textures.enemy_cannon_crashed.clone();
        }
    }
}
