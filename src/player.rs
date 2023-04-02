use std::f32::consts::PI;

use crate::actions::Actions;
use crate::environment::{Collidable, MAP_HEIGHT, MAP_WIDTH};
use crate::loading::TextureAssets;
use crate::menu::MainCamera;
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub const PLAYER_HEIGHT: f32 = 64.;
pub const PLAYER_WIDTH: f32 = 28.;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

// TODO move this into own plugin
#[derive(Component)]
pub struct Movement {
    speed: f32,
    vector: Vec2,
}

impl Default for Movement {
    fn default() -> Self {
        Movement {
            speed: 150.0,
            vector: Vec2::new(0., 1.),
        }
    }
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_system(move_player.in_set(OnUpdate(GameState::Playing)))
            .add_system(detect_collisions.in_set(OnUpdate(GameState::Playing)))
            .add_system(
                camera_follow_player
                    .in_set(OnUpdate(GameState::Playing))
                    .after(move_player),
            )
            .add_system(continuous_movement.in_set(OnUpdate(GameState::Playing)))
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
            transform: Transform::from_translation(Vec3::new(center_x, center_y, 2.))
                .with_rotation(Quat::from_rotation_z(0.)),
            // transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.)),
            ..Default::default()
        })
        .insert(Player)
        .insert(Movement {
            ..Default::default()
        });
}

fn continuous_movement(
    time: Res<Time>,
    mut movement_query: Query<(&mut Transform, &Movement), With<Movement>>,
) {
    // TODO apply to all the things
    let (mut transform, movement) = movement_query.get_single_mut().unwrap();
    transform.translation += Vec3::new(
        movement.vector.x * movement.speed * time.delta_seconds(),
        movement.vector.y * movement.speed * time.delta_seconds(),
        0.,
    );
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
        player_translation.y,
        camera_transform.translation.z,
    ));
}

fn detect_collisions(
    collidables_query: Query<(&Transform, &Collidable)>,
    mut player_query: Query<(&Transform, &mut Handle<Image>), With<Player>>,
    textures: Res<TextureAssets>,
    mut state: ResMut<NextState<GameState>>,
) {
    let (player_transform, mut texture_handle) = player_query.get_single_mut().unwrap();
    let player_size = Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT);
    for (collidable_transform, collidable) in collidables_query.iter() {
        // let distance = player_transform.translation.distance_squared(collidable.translation);
        //  if distance < 64. {
        //      state.set(GameState::Menu);
        //  }
        let collision = collide(
            player_transform.translation,
            player_size,
            collidable_transform.translation,
            collidable.size,
        );
        if Option::is_some(&collision) {
            *texture_handle = textures.boat_crashed.clone();
            state.set(GameState::End);
        }
    }
}
