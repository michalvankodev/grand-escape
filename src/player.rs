use crate::actions::Actions;
use crate::environment::{MAP_WIDTH, MAP_HEIGHT};
use crate::loading::TextureAssets;
use crate::GameState;
use crate::menu::MainCamera;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_system(move_player.in_set(OnUpdate(GameState::Playing)))
            .add_system(camera_follow_player.in_set(OnUpdate(GameState::Playing)).after(move_player));
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>, assets: Res<Assets<Image>>) {
    // Spawn player to the center of the map
    let texture = textures.boat.clone();
    let texture_descriptor = &assets.get(&texture).unwrap().texture_descriptor;
    let texture_width = texture_descriptor.size.width;
    let texture_height = texture_descriptor.size.height;
    let center_x = MAP_WIDTH / 2; //- texture_width / 2;
    let center_y = MAP_HEIGHT / 2; //- texture_height / 2;
    info!("center = {} {}", center_x, center_y);

    commands
        .spawn(SpriteBundle {
            texture: textures.boat.clone(),
            transform: Transform::from_translation(Vec3::new(center_x as f32, center_y as f32, 2.)),
            // transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.)),
            ..Default::default()
        })
        .insert(Player);
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}

fn camera_follow_player(
    commands: Commands,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
  let player_translation = player_query.get_single().unwrap().translation;
  let mut camera_transform = camera_query.get_single_mut().unwrap();
  *camera_transform = Transform::from_translation(Vec3::new((MAP_WIDTH / 2) as f32, player_translation.y, camera_transform.translation.z));
}
