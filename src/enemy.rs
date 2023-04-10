use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use rand::Rng;

use crate::{
    difficulty::Difficulty,
    environment::{Collidable, LAND_TILE_SIZE, MAP_WIDTH},
    health::{Bullet, Health, Mass},
    loading::{AudioAssets, TextureAssets},
    menu::MainCamera,
    player::{Movement, Player},
    score::GameScore,
    GameState,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimers>()
            .add_system(spawn_enemies_onside.in_set(OnUpdate(GameState::Playing)))
            .add_system(spawn_pirates.in_set(OnUpdate(GameState::Playing)))
            .add_system(despawn_enemies.in_schedule(OnEnter(GameState::Restart)))
            .add_system(enemies_shoot_at_player.in_set(OnUpdate(GameState::Playing)))
            .add_system(pirates_shoot_at_player.in_set(OnUpdate(GameState::Playing)))
            .add_system(despawn_enemies_out_of_sight.in_set(OnUpdate(GameState::Playing)))
            .add_system(detect_killed_enemies.in_set(OnUpdate(GameState::Playing)))
            .add_system(enemies_face_player.in_set(OnUpdate(GameState::Playing)))
            .add_system(pirate_cannons_face_player.in_set(OnUpdate(GameState::Playing)))
            .add_system(increase_difficulty_medium.in_schedule(OnEnter(Difficulty::Medium)))
            .add_system(increase_difficulty_hard.in_schedule(OnEnter(Difficulty::Hard)));
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimers {
    side_cannons: Vec<Timer>,
    pirate_ships: Vec<Timer>,
}

impl Default for EnemySpawnTimers {
    fn default() -> Self {
        EnemySpawnTimers {
            side_cannons: vec![Timer::new(Duration::from_secs(5), TimerMode::Repeating)],
            pirate_ships: vec![],
        }
    }
}

#[derive(Component)]
pub struct Enemy {
    vector: Vec2,
    shooting_timer: Timer,
    is_alive: bool,
}

#[derive(Component)]
pub struct EnemyPirate;

#[derive(Component)]
pub struct EnemyPirateCannon {
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

pub const PIRATE_SIZE: Vec2 = Vec2::new(32., 64.);

fn spawn_enemies_onside(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timers: ResMut<EnemySpawnTimers>,
    textures: Res<TextureAssets>,
    camera_query: Query<&Transform, With<MainCamera>>,
) {
    let camera_position = camera_query.get_single().unwrap().translation.y;
    let next_spawn_position = camera_position + 600.;
    for timer in &mut spawn_timers.side_cannons {
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
                    ..Default::default()
                })
                .insert(Enemy {
                    ..Default::default()
                })
                .insert(Health {
                    max_health: 2,
                    health_amount: 2,
                    size: Vec2::new(64., 64.),
                    immune_to_bullets: false,
                    mass: Mass::Wood,
                });
            let mut rng = rand::thread_rng();
            let duration = rng.gen_range(4500..7000);
            timer.set_duration(Duration::from_millis(duration));
        }
    }
}

fn spawn_pirates(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timers: ResMut<EnemySpawnTimers>,
    textures: Res<TextureAssets>,
    camera_query: Query<&Transform, With<MainCamera>>,
) {
    let camera_position = camera_query.get_single().unwrap().translation.y;
    let next_spawn_position = camera_position + 600.;
    for timer in &mut spawn_timers.pirate_ships {
        timer.tick(time.delta());
        if timer.finished() {
            let position = get_random_pirate_spawn_position();
            let mut rng = rand::thread_rng();
            let random_angle = rng.gen_range(0.0..2. * PI);
            commands
                .spawn(SpriteBundle {
                    texture: textures.enemy_pirate1.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        position,
                        next_spawn_position,
                        4.,
                    ))
                    .with_rotation(Quat::from_rotation_z(random_angle)),
                    ..Default::default()
                })
                .insert(EnemyPirate)
                .insert(Health {
                    max_health: 3,
                    health_amount: 3,
                    size: PIRATE_SIZE,
                    immune_to_bullets: false,
                    mass: Mass::Wood,
                })
                .insert(Collidable {
                    damage: 5,
                    size: PIRATE_SIZE,
                    is_alive: true,
                })
                .with_children(|parent| {
                    parent
                        .spawn(SpriteBundle {
                            texture: textures.boat_cannon.clone(),
                            transform: Transform::from_translation(Vec3::new(0., 20., 5.1))
                                .with_rotation(Quat::from_rotation_z(0.)),
                            ..Default::default()
                        })
                        .insert(EnemyPirateCannon {
                            vector: Vec2::new(0., 0.),
                            shooting_timer: Timer::new(
                                Duration::from_millis(2000),
                                TimerMode::Repeating,
                            ),
                            is_alive: true,
                        });
                });
            let duration = rng.gen_range(13000..17000);
            timer.set_duration(Duration::from_millis(duration));
        }
    }
}

pub fn get_random_pirate_spawn_position() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(40.0..MAP_WIDTH - 40.)
    // TODO Check if collides with obstacle
}

fn get_random_spawn_position() -> SpawnPosition {
    let mut rng = rand::thread_rng();
    if rng.gen::<bool>() {
        SpawnPosition::Left
    } else {
        SpawnPosition::Right
    }
}

pub fn despawn_enemies_out_of_sight(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), Or<(With<Enemy>, With<EnemyPirate>)>>,
    camera_query: Query<&Transform, With<MainCamera>>,
) {
    let camera_translation = camera_query.get_single().unwrap().translation;
    for (enemy_entity, transform) in enemy_query.iter() {
        if transform.translation.y < camera_translation.y - 600. {
            commands.entity(enemy_entity).despawn_recursive();
        }
    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, Or<(With<Enemy>, With<EnemyPirate>)>>,
    mut spawn_timers: ResMut<EnemySpawnTimers>,
) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn_recursive();
    }
    *spawn_timers = EnemySpawnTimers::default();
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

fn pirate_cannons_face_player(
    mut transform_query: Query<
        (
            &mut Transform,
            &GlobalTransform,
            &mut EnemyPirateCannon,
            &Parent,
        ),
        Without<Player>,
    >,
    parent_query: Query<&Transform, (Without<Player>, Without<EnemyPirateCannon>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_translation = player_query.get_single().unwrap().translation;

    for (mut cannon_transform, cannon_global_transform, mut enemy_cannon, enemy_parent) in
        transform_query.iter_mut()
    {
        if enemy_cannon.is_alive == false {
            continue;
        }
        let parent_transform = parent_query.get(enemy_parent.get()).unwrap();
        let parent_angle = parent_transform.rotation.to_euler(EulerRot::YXZ);

        let mut vector = (player_translation - cannon_global_transform.translation()).truncate();
        vector.y = vector.y + 50.;

        enemy_cannon.vector = vector.normalize();

        let angle = enemy_cannon.vector.y.atan2(enemy_cannon.vector.x) - PI / 2.0;
        cannon_transform.rotation = Quat::from_rotation_z(angle - parent_angle.2);
    }
}

fn enemies_shoot_at_player(
    mut commands: Commands,
    mut shooters_query: Query<(&mut Enemy, &Transform, Entity)>,
    time: Res<Time>,
    textures: Res<TextureAssets>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
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
            audio
                .play(audio_assets.bullet_fire.clone())
                .with_volume(0.3);
        }
    }
}

fn pirates_shoot_at_player(
    mut commands: Commands,
    mut shooters_query: Query<(&mut EnemyPirateCannon, &GlobalTransform, &Parent)>,
    time: Res<Time>,
    textures: Res<TextureAssets>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    for (mut enemy_cannon, global_transform, enemy_pirate) in shooters_query.iter_mut() {
        enemy_cannon.shooting_timer.tick(time.delta());
        info!("Pirate shooting check");
        if enemy_cannon.is_alive == false {
            continue;
        }
        if enemy_cannon.shooting_timer.finished() {
            info!("shooting");
            let enemy_translation = global_transform.translation().truncate();
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
                .insert(Bullet::new(enemy_pirate.get()))
                .insert(Movement {
                    vector: enemy_cannon.vector,
                    speed: 300.0,
                    ..Default::default()
                });
            audio
                .play(audio_assets.bullet_fire.clone())
                .with_volume(0.3);
        }
    }
}

fn detect_killed_enemies(
    mut enemies_q: Query<(&mut Enemy, &mut Handle<Image>, &Health)>,
    textures: Res<TextureAssets>,
    mut game_score: ResMut<GameScore>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    for (mut enemy, mut handle, health) in enemies_q.iter_mut() {
        if enemy.is_alive == false {
            continue;
        }
        if health.health_amount <= 0 {
            enemy.is_alive = false;
            *handle = textures.enemy_cannon_crashed.clone();
            game_score.score += 10;
            audio.play(audio_assets.boat_crash.clone()).with_volume(0.1);
        }
    }
}

fn increase_difficulty_medium(mut spawn_timers: ResMut<EnemySpawnTimers>) {
    spawn_timers
        .pirate_ships
        .push(Timer::new(Duration::from_secs(5), TimerMode::Repeating));
}

fn increase_difficulty_hard(mut spawn_timers: ResMut<EnemySpawnTimers>) {
    spawn_timers
        .pirate_ships
        .push(Timer::new(Duration::from_secs(5), TimerMode::Repeating));
    spawn_timers
        .side_cannons
        .push(Timer::new(Duration::from_secs(5), TimerMode::Repeating));
}
