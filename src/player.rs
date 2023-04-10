use std::f32::consts::{FRAC_PI_2, PI};
use std::time::Duration;

use crate::actions::Actions;
use crate::environment::{Collidable, MAP_HEIGHT, MAP_WIDTH};
use crate::health::{Bullet, Health, Mass};
use crate::loading::{TextureAssets, AudioAssets};
use crate::menu::MainCamera;
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;
use bevy_kira_audio::{Audio, AudioControl};

pub const PLAYER_HEIGHT: f32 = 64.;
pub const PLAYER_WIDTH: f32 = 28.;
pub const PLAYER_SIZE: Vec2 = Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT);


pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCannon {
    pub vector: Vec2,
    pub timer: Timer,
    pub turn_rate: f32, // how many degrees we allow to turn cannon in one second
}

// TODO move this into own plugin
#[derive(Component)]
pub struct Movement {
    pub speed: f32,
    pub vector: Vec2,
}

impl Default for Movement {
    fn default() -> Self {
        Movement {
            speed: 120.0,
            vector: Vec2::new(0., 1.),
        }
    }
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(GameState::Init)))
            .add_system(despawn_player.in_schedule(OnEnter(GameState::Restart)))
            .add_system(move_player.in_set(OnUpdate(GameState::Playing)))
            .add_system(detect_collisions.in_set(OnUpdate(GameState::Playing)))
            .add_system(
                camera_follow_player
                    .in_set(OnUpdate(GameState::Playing))
                    .after(move_player),
            )
            .add_system(continuous_movement.in_set(OnUpdate(GameState::Playing)))
            .add_system(move_player_cannon.in_set(OnUpdate(GameState::Playing)))
            .add_system(player_shoot.in_set(OnUpdate(GameState::Playing)))
            .add_system(detect_player_dead.in_set(OnUpdate(GameState::Playing)))
            .add_system(display_boat_damage.in_set(OnUpdate(GameState::Playing)))
            .add_system(rotate_transform_to_movement.in_set(OnUpdate(GameState::Playing)));
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    // Spawn player to the center of the map
    let center_x = MAP_WIDTH / 2.;
    let center_y = MAP_HEIGHT / 2.;

    commands
        .spawn(SpriteBundle {
            texture: textures.boat.clone(),
            transform: Transform::from_translation(Vec3::new(center_x, center_y, 5.))
                .with_rotation(Quat::from_rotation_z(0.)),
            ..Default::default()
        })
        .insert(Player)
        .insert(Health {
            max_health: 10,
            health_amount: 10,
            size: Vec2::new(28., 64.),
            immune_to_bullets: false,
            mass: Mass::Wood,
        })
        .insert(Movement {
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    texture: textures.boat_cannon.clone(),
                    transform: Transform::from_translation(Vec3::new(0., 20., 5.1))
                        .with_rotation(Quat::from_rotation_z(0.)),
                    ..Default::default()
                })
                .insert(PlayerCannon {
                    vector: Vec2::new(0., 0.),
                    timer: Timer::new(Duration::from_millis(1000), TimerMode::Once),
                    turn_rate: 3. * FRAC_PI_2,
                });
        });
}

fn despawn_player(mut commands: Commands, player_q: Query<Entity, With<Player>>) {
    let entity = player_q.get_single().unwrap();
    commands.entity(entity).despawn_recursive();
}

fn continuous_movement(
    time: Res<Time>,
    mut movement_query: Query<(&mut Transform, &Movement), With<Movement>>,
) {
    for (mut transform, movement) in movement_query.iter_mut() {
        transform.translation += Vec3::new(
            movement.vector.x * movement.speed * time.delta_seconds(),
            movement.vector.y * movement.speed * time.delta_seconds(),
            0.,
        );
    }
}

fn rotate_transform_to_movement(mut transform_query: Query<(&mut Transform, &Movement)>) {
    for (mut transform, movement) in transform_query.iter_mut() {
        let angle = movement.vector.x * PI / 2.;
        transform.rotation = Quat::from_rotation_z(-angle);
        // info!("rotation {}, vector {}, translation {}", angle, movement.vector, transform.translation.truncate());
    }
}

// TODO breaking optional
fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Movement, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let turn_rate = 1.8;
    let movement = Vec2::new(
        actions.player_movement.unwrap().x * turn_rate * time.delta_seconds(),
        actions.player_movement.unwrap().y * turn_rate * time.delta_seconds(),
    );
    for mut player_movement in &mut player_query {
        player_movement.vector += movement;
        player_movement.vector = player_movement.vector.normalize()
    }
}

fn camera_follow_player(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_translation = player_query.get_single().unwrap().translation;
    let mut camera_transform = camera_query.get_single_mut().unwrap();
    *camera_transform = Transform::from_translation(Vec3::new(
        MAP_WIDTH / 2.,
        player_translation.y + 180.,
        camera_transform.translation.z,
    ));
}

fn detect_collisions(
    mut player_q: Query<(&Transform, &mut Health), With<Player>>,
    mut collidables_query: Query<(&Transform, &Collidable, Option<&mut Health>), Without<Player>>,
) {
    let (player_transform, mut player_health) = player_q.get_single_mut().unwrap();
    for (collidable_transform, collidable, collidable_health) in collidables_query.iter_mut() {
        if collidable.is_alive == false {
            continue;
        }
        let collision = collide(
            player_transform.translation,
            PLAYER_SIZE,
            collidable_transform.translation,
            collidable.size,
        );
        if Option::is_some(&collision) {
            player_health.health_amount -= collidable.damage;
            if let Some(mut col_health) = collidable_health {
                col_health.health_amount -= 1;
            }
        }
    }
}

fn display_boat_damage(
    mut player_q: Query<(&mut Handle<Image>, &Health), With<Player>>,
    textures: Res<TextureAssets>,
) {
    let (mut handle, health) = player_q.get_single_mut().unwrap();
    let health_percentage = health.health_amount as f32 / health.max_health as f32 * 100.;
    let next_texture = if health_percentage <= 0. {
        &textures.boat_crashed
    } else if health_percentage <= 33. {
        &textures.boat_dmg2
    } else if health_percentage <= 66. {
        &textures.boat_dmg1
    } else {
        &textures.boat
    };

    if *handle != *next_texture {
        *handle = next_texture.clone();
    }
}

fn detect_player_dead(
    player_health_q: Query<&Health, With<Player>>,
    mut state: ResMut<NextState<GameState>>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    let player_health = player_health_q.get_single().unwrap();
    if player_health.health_amount <= 0 {
        audio.play(audio_assets.boat_crash.clone()).with_volume(0.5);
        state.set(GameState::End);
    }
}

fn move_player_cannon(
    mut cannon_query: Query<(&mut Transform, &GlobalTransform, &mut PlayerCannon), Without<Player>>,
    mut player_query: Query<&Transform, With<Player>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    time: Res<Time>,
) {
    let window = window.get_single().unwrap();
    let (camera, camera_transform) = camera_q.single();
    let (mut cannon_transform, global_cannon_transform, mut cannon) =
        cannon_query.get_single_mut().unwrap();
    let player_transform = player_query.get_single_mut().unwrap();
    let player_rotation = player_transform.rotation;

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(position) = window
        .cursor_position()
        .and_then(|pos| camera.viewport_to_world(camera_transform, pos))
        .map(|ray| ray.origin.truncate())
    {
        // Apply player rotation to the current cannon rotation
        let current_angle = cannon_transform.rotation.to_euler(EulerRot::YXZ);
        let player_angle = player_rotation.to_euler(EulerRot::YXZ);

        // Vector where we are pointing at
        let wishful_vector = position - global_cannon_transform.translation().truncate();
        let wishful_vector_normalized = wishful_vector.normalize();

        let wishful_angle = wishful_vector_normalized
            .y
            .atan2(wishful_vector_normalized.x)
            - PI / 2.;
        // Nice Michal PogChamp
        let wishful_angle = if wishful_angle < -PI {
            wishful_angle + 2. * PI
        } else {
            wishful_angle
        };

        let wishful_rotation = Quat::from_rotation_z(wishful_angle - player_angle.2);
        let wishful_rotation_angle = wishful_rotation.to_euler(EulerRot::YXZ);

        let angle_sign = if (wishful_rotation_angle.2 - current_angle.2).abs() < PI {
            if current_angle.2 >= wishful_rotation_angle.2 {
                -1.
            } else {
                1.
            }
        } else {
            if current_angle.2 >= wishful_rotation_angle.2 {
                1.
            } else {
                -1.
            }
        };

        let angle_add = cannon.turn_rate * time.delta_seconds() * angle_sign;
        let next_rotation = Quat::from_rotation_z(current_angle.2 + angle_add);

        cannon.vector = angle_to_vector(current_angle.2 + player_angle.2 + angle_add);
        cannon_transform.rotation = next_rotation;
    }
}

fn player_shoot(
    mut commands: Commands,
    mut player_cannon_q: Query<(&GlobalTransform, &mut PlayerCannon)>,
    player_q: Query<Entity, With<Player>>,
    // mouse_input: EventReader<MouseButtonInput>,
    mouse_input: Res<Input<MouseButton>>,
    time: Res<Time>,
    textures: Res<TextureAssets>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    let (cannon_transform, mut player_cannon) = player_cannon_q.get_single_mut().unwrap();
    let player = player_q.get_single().unwrap();
    player_cannon.timer.tick(time.delta());
    if mouse_input.pressed(MouseButton::Left) {
        if player_cannon.timer.finished() {
            let player_cannon_translation = cannon_transform.translation().truncate();
            commands
                .spawn(SpriteBundle {
                    texture: textures.bullet.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        player_cannon_translation.x,
                        player_cannon_translation.y,
                        3.,
                    )),
                    ..Default::default()
                })
                    // TODO 
                .insert(Bullet::new(player))
                .insert(Movement {
                    vector: player_cannon.vector,
                    speed: 350.0,
                    ..Default::default()
                });
            player_cannon.timer.reset();
            audio.play(audio_assets.bullet_fire.clone()).with_volume(0.7);
        }
    }
}

fn angle_to_vector(angle: f32) -> Vec2 {
    let angle = angle + PI / 2.;
    let x = angle.cos();
    let y = angle.sin();
    Vec2::new(x, y)
}
