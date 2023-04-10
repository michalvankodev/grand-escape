use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_kira_audio::{Audio, AudioControl};

use crate::{environment::MAP_WIDTH, loading::AudioAssets, GameState};

pub enum Mass {
    Wood,
    Rock,
}

#[derive(Component)]
pub struct Health {
    pub max_health: i32,
    pub health_amount: i32,
    pub size: Vec2,
    pub immune_to_bullets: bool,
    pub mass: Mass,
}

#[derive(Component)]
pub struct Bullet {
    pub shooter: Entity,
    pub damage: i32,
    pub size: Vec2,
}

impl Bullet {
    pub fn new(shooter: Entity) -> Self {
        Bullet {
            shooter,
            damage: 1,
            size: Vec2::new(5., 5.),
        }
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(detect_bullet_collisions.in_set(OnUpdate(GameState::Playing)))
            .add_system(despawn_blind_bullets.in_set(OnUpdate(GameState::Playing)))
            .add_system(despawn_bullets.in_schedule(OnEnter(GameState::Restart)));
    }
}

fn detect_bullet_collisions(
    mut commands: Commands,
    bullets_query: Query<(Entity, &Transform, &Bullet)>,
    mut health_query: Query<(&Transform, &mut Health, Entity)>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    for (bullet_entity, bullet_transform, bullet) in bullets_query.iter() {
        for (health_transform, mut health, entity) in health_query.iter_mut() {
            if bullet.shooter == entity || health.immune_to_bullets {
                continue;
            }
            let collision = collide(
                bullet_transform.translation,
                bullet.size,
                health_transform.translation,
                health.size,
            );
            if Option::is_some(&collision) {
                commands.entity(bullet_entity).despawn();
                health.health_amount = health.health_amount - bullet.damage;
                match health.mass {
                    Mass::Wood => {
                        audio.play(audio_assets.bullet_hit.clone()).with_volume(0.4);
                    }
                    Mass::Rock => {
                        audio
                            .play(audio_assets.bullet_hit_rock.clone())
                            .with_volume(0.8);
                    }
                }
            }
        }
    }
}

fn despawn_blind_bullets(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (bullet_entity, transform) in bullet_query.iter() {
        if transform.translation.x < -100. || transform.translation.x > MAP_WIDTH + 100. {
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn despawn_bullets(mut commands: Commands, bullet_query: Query<Entity, With<Bullet>>) {
    for bullet_entity in bullet_query.iter() {
        commands.entity(bullet_entity).despawn();
    }
}
