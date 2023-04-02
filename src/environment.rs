use std::f32::consts::PI;

use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use rand::seq::SliceRandom;

pub const MAP_WIDTH: f32 = 512.;
pub const MAP_HEIGHT: f32 = 576.;

pub struct EnvironmentPlugin;

#[derive(Component)]
pub struct BorderTile;

#[derive(Component)]
pub struct WaterTile;

#[derive(Clone)]
enum MapObject {}

const WATER_TILE_SIZE: f32 = 64.;
const BORDER_TILE_HEIGHT: f32 = 64.;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init_water.in_schedule(OnEnter(GameState::Playing)))
            .add_system(init_border.in_schedule(OnEnter(GameState::Playing)));
    }
}

fn init_water(mut commands: Commands, textures: Res<TextureAssets>) {
    let water_tile_x_positions = (0..(MAP_WIDTH / WATER_TILE_SIZE) as u32)
        .map(|x| x as f32 * WATER_TILE_SIZE + WATER_TILE_SIZE / 2.);
    let water_tile_y_positions = (0..(MAP_HEIGHT / WATER_TILE_SIZE) as u32)
        .map(|y| y as f32 * WATER_TILE_SIZE + WATER_TILE_SIZE / 2.);
    let map_matrix = water_tile_x_positions
        .flat_map(move |x| water_tile_y_positions.clone().map(move |y| (x, y)));
    // map_matrix.collect::<Vec<(u32,u32)>>().iter().for_each(|(x,y)| {
    map_matrix.for_each(|(x, y)| {
        commands
            .spawn(SpriteBundle {
                texture: textures.water_tile.clone(),
                transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 1.)),
                ..Default::default()
            })
            .insert(WaterTile);
    });
}

fn init_border(mut commands: Commands, textures: Res<TextureAssets>) {
    let y_positions = (0..(MAP_HEIGHT / BORDER_TILE_HEIGHT) as u32)
        .map(|y| y as f32 * BORDER_TILE_HEIGHT - BORDER_TILE_HEIGHT / 2.);
    let border_tiles = [
        textures.border_tile1.clone(),
        textures.border_tile2.clone(),
        textures.border_tile3.clone(),
        textures.border_tile4.clone(),
    ];
    let mut rng = rand::thread_rng();
    // Left side
    y_positions.clone().for_each(|y| {
        commands
            .spawn(SpriteBundle {
                texture: border_tiles.choose(&mut rng).unwrap().clone(),
                transform: Transform::from_translation(Vec3::new(
                    0. + BORDER_TILE_HEIGHT / 2.,
                    y,
                    1.1,
                )).with_rotation(Quat::from_rotation_z(PI)),
                ..Default::default()
            })
            .insert(BorderTile);
    });
    // Left side
    y_positions.for_each(|y| {
        commands
            .spawn(SpriteBundle {
                texture: border_tiles.choose(&mut rng).unwrap().clone(),
                transform: Transform::from_translation(Vec3::new(
                    MAP_WIDTH - BORDER_TILE_HEIGHT / 2., // TODO Width
                    y,
                    1.1,
                )),
                ..Default::default()
            })
            .insert(BorderTile);
    })
}
