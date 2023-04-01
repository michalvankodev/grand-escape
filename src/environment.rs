use std::ops::Range;

use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub const MAP_WIDTH: u32 = 512;
pub const MAP_HEIGHT: u32 = 576;

pub struct EnvironmentPlugin;

#[derive(Component)]
pub struct WaterTile;

#[derive(Clone)]
enum MapObject {}

#[derive(Clone)]
struct MapDefinition {
    width: u32,
    height: u32,
    map_objects: Vec<Vec<MapObject>>,
}

impl MapDefinition {
    fn new() -> MapDefinition {
        MapDefinition {
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            map_objects: vec![],
        }
    }
}

const TILE_SIZE: u32 = 64;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init_water.in_schedule(OnEnter(GameState::Playing)));
    }
}

fn init_water(mut commands: Commands, textures: Res<TextureAssets>) {
    let map_def = MapDefinition::new();
    let water_tile_x_positions = (0..map_def.width / TILE_SIZE).map(|x| x * TILE_SIZE + TILE_SIZE / 2);
    let water_tile_y_positions = (0..map_def.height / TILE_SIZE).map(|y| y * TILE_SIZE + TILE_SIZE / 2);
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
