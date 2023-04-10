use std::time::Duration;

use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_kira_audio::{Audio, AudioControl};
use rand::Rng;

use crate::{
    environment::Collidable,
    health::{Health, Mass},
    loading::{AudioAssets, TextureAssets},
    menu::MainCamera,
    obstacle::get_random_obstacle_spawn_position,
    player::{Player, PlayerCannon, PLAYER_SIZE},
    GameState,
};

pub struct PowerUpPlugin;
impl Plugin for PowerUpPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PowerUpSpawnTimers>()
            .init_resource::<PowerUpExhaustTimers>()
            .add_system(spawn_power_up_barrels.in_set(OnUpdate(GameState::Playing)))
            .add_system(detect_dead_barrels.in_set(OnUpdate(GameState::Playing)))
            .add_system(pick_up_power_ups.in_set(OnUpdate(GameState::Playing)))
            .add_system(tick_exhaust_timers.in_set(OnUpdate(GameState::Playing)))
            .add_system(despawn_power_ups.in_schedule(OnEnter(GameState::Restart)));
    }
}

const BARREL_SIZE: Vec2 = Vec2::new(32., 24.);
const POWER_UP_SIZES: [bevy::prelude::Vec2; 2] = [Vec2::new(45., 40.), Vec2::new(45., 40.)];

const POWER_UP_KINDS: [PowerUpType; 2] = [PowerUpType::Repair, PowerUpType::Weapon];

#[derive(Resource)]
pub struct PowerUpSpawnTimers {
    timers: Vec<Timer>,
}

impl Default for PowerUpSpawnTimers {
    fn default() -> Self {
        PowerUpSpawnTimers {
            timers: vec![Timer::new(Duration::from_secs(15), TimerMode::Repeating)],
        }
    }
}

#[derive(Resource)]
pub struct PowerUpExhaustTimers {
    pub weapon: Vec<Timer>,
}

impl Default for PowerUpExhaustTimers {
    fn default() -> Self {
        PowerUpExhaustTimers { weapon: vec![] }
    }
}

#[derive(Component)]
pub struct PowerUpBarrel;

#[derive(Clone, Copy)]
pub enum PowerUpType {
    Repair,
    Weapon,
}

#[derive(Component)]
pub struct PowerUp {
    kind: PowerUpType,
}

fn spawn_power_up_barrels(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timers: ResMut<PowerUpSpawnTimers>,
    textures: Res<TextureAssets>,
    camera_query: Query<&Transform, With<MainCamera>>,
) {
    let camera_position = camera_query.get_single().unwrap().translation.y;
    let next_spawn_position = camera_position + 600.;
    for timer in &mut spawn_timers.timers {
        timer.tick(time.delta());
        if timer.finished() {
            let position = get_random_obstacle_spawn_position();
            commands
                .spawn(SpriteBundle {
                    texture: textures.barrel.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        position,
                        next_spawn_position,
                        2.,
                    )),
                    ..Default::default()
                })
                .insert(PowerUpBarrel)
                .insert(Health {
                    max_health: 1,
                    health_amount: 1,
                    size: BARREL_SIZE,
                    immune_to_bullets: false,
                    mass: Mass::Wood,
                })
                .insert(Collidable {
                    size: BARREL_SIZE,
                    damage: 1,
                    is_alive: true,
                });
            let mut rng = rand::thread_rng();
            let duration = rng.gen_range(10000..20000);
            timer.set_duration(Duration::from_millis(duration));
        }
    }
}

fn despawn_power_ups(
    mut commands: Commands,
    power_up_q: Query<Entity, Or<(With<PowerUp>, With<PowerUpBarrel>)>>,
    mut power_up_spawn_timers: ResMut<PowerUpSpawnTimers>,
    mut exhaust_timers: ResMut<PowerUpExhaustTimers>,
) {
    for entity in power_up_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
    *power_up_spawn_timers = PowerUpSpawnTimers::default();
    *exhaust_timers = PowerUpExhaustTimers::default();
}

fn detect_dead_barrels(
    mut commands: Commands,
    barrel_q: Query<(Entity, &Transform, &Health), With<PowerUpBarrel>>,
    textures: Res<TextureAssets>,
) {
    for (entity, transform, health) in barrel_q.iter() {
        if health.health_amount <= 0 {
            let available_power_ups = [
                textures.power_up_health.clone(),
                textures.power_up_weapon.clone(),
            ];
            let mut rng = rand::thread_rng();
            let which_one_index = rng.gen_range(0..2);
            let power_up_texture = &available_power_ups[which_one_index];
            let size = POWER_UP_SIZES[which_one_index];
            let kind = POWER_UP_KINDS[which_one_index];

            commands
                .spawn(SpriteBundle {
                    texture: power_up_texture.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        transform.translation.x,
                        transform.translation.y,
                        2.,
                    )),
                    ..Default::default()
                })
                .insert(PowerUp { kind })
                .insert(Health {
                    max_health: 1,
                    health_amount: 1,
                    size,
                    immune_to_bullets: true,
                    mass: Mass::Wood,
                })
                .insert(Collidable {
                    size,
                    damage: 0,
                    is_alive: true,
                });

            commands.entity(entity).despawn();
        }
    }
}

fn pick_up_power_ups(
    mut commands: Commands,
    mut player_q: Query<(&Transform, &mut Health), With<Player>>,
    mut player_cannon_q: Query<&mut PlayerCannon>,
    power_ups_q: Query<(Entity, &Transform, &Collidable, &PowerUp), Without<Player>>,
    mut power_ups_exhaust_timers: ResMut<PowerUpExhaustTimers>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    let (player_transform, mut player_health) = player_q.get_single_mut().unwrap();
    let mut player_cannon = player_cannon_q.get_single_mut().unwrap();
    for (entity, transform, collidable, power_up) in power_ups_q.iter() {
        let collision = collide(
            player_transform.translation,
            PLAYER_SIZE,
            transform.translation,
            collidable.size,
        );
        if Option::is_some(&collision) {
            match power_up.kind {
                PowerUpType::Repair => {
                    player_health.max_health += 1;
                    player_health.health_amount =
                        (player_health.health_amount + 3).min(player_health.max_health);
                    audio.play(audio_assets.repair.clone()).with_volume(0.4);
                }
                PowerUpType::Weapon => {
                    let current_timer_duration = player_cannon.timer.duration().as_millis();
                    let current_turn_rate = player_cannon.turn_rate;
                    player_cannon.timer.set_duration(Duration::from_millis(
                        (current_timer_duration as f32 * 0.75) as u64,
                    ));
                    player_cannon.turn_rate = current_turn_rate * 1.25;
                    power_ups_exhaust_timers
                        .weapon
                        .push(Timer::new(Duration::from_secs(7), TimerMode::Once));
                    audio
                        .play(audio_assets.power_up_weapon.clone())
                        .with_volume(0.7);
                }
            }
            commands.entity(entity).despawn();
        }
    }
}

fn tick_exhaust_timers(
    mut exhaust_timers: ResMut<PowerUpExhaustTimers>,
    mut player_cannon_q: Query<&mut PlayerCannon>,
    time: Res<Time>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    let mut player_cannon = player_cannon_q.get_single_mut().unwrap();
    for timer in exhaust_timers.weapon.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let current_timer_duration = player_cannon.timer.duration().as_millis();
            let current_turn_rate = player_cannon.turn_rate;
            // Side-effect of picking up power up is that it will improve the weapon over time
            // even when power up is not active anymore
            // Mathematically upgrading by 25% and then downgrading by 25% will not equal 100%
            player_cannon.timer.set_duration(Duration::from_millis(
                (current_timer_duration as f32 * 1.25) as u64,
            ));
            player_cannon.turn_rate = current_turn_rate * 0.75;
            audio
                .play(audio_assets.power_up_weapon_exhaust.clone())
                .with_volume(0.7);
        }
    }
    exhaust_timers.weapon.retain(|timer| !timer.finished());
}
