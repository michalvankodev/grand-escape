use std::f32::consts::PI;

use crate::loading::TextureAssets;
use crate::menu::MainCamera;
use crate::GameState;
use bevy::prelude::*;
use rand::seq::SliceRandom;

pub const MAP_WIDTH: f32 = 512.;
pub const MAP_HEIGHT: f32 = 576.;

pub struct EnvironmentPlugin;

#[derive(Component)]
pub struct Collidable {
    pub size: Vec2,
    pub damage: i32,
}

#[derive(Component)]
pub struct LandTile;

#[derive(Component)]
pub struct BorderTile;

#[derive(Component)]
pub struct WaterTile;

#[derive(Resource)]
pub struct MapObject {
    water_top: f32,
    border_top: f32,
    land_top: f32,
}

impl Default for MapObject {
    fn default() -> Self {
        MapObject {
            water_top: 0.,
            border_top: 0.,
            land_top: 0.,
        }
    }
}

pub const LAND_TILE_SIZE: f32 = 64.;
pub const WATER_TILE_SIZE: f32 = 64.;
pub const BORDER_TILE_HEIGHT: f32 = 64.;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapObject>()
            .add_system(spawn_water.in_set(OnUpdate(GameState::Playing)))
            .add_system(spawn_border.in_set(OnUpdate(GameState::Playing)))
            .add_system(spawn_land.in_set(OnUpdate(GameState::Playing)))
            .add_system(despawn_environment.in_schedule(OnEnter(GameState::Restart)));
    }
}

fn spawn_water(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    camera_query: Query<&Transform, With<MainCamera>>,
    mut map_object: ResMut<MapObject>,
) {
    let camera_transform = camera_query.get_single().unwrap();
    let y_where_water_should_be_generated = camera_transform.translation.y + 800.;

    if map_object.water_top > y_where_water_should_be_generated {
        return;
    }

    let water_tile_x_positions = (0..(MAP_WIDTH / WATER_TILE_SIZE) as u32)
        .map(|x| x as f32 * WATER_TILE_SIZE + WATER_TILE_SIZE / 2.);
    let water_tile_y_position = map_object.water_top + WATER_TILE_SIZE;

    let map_matrix = water_tile_x_positions.map(move |x| (x, water_tile_y_position));
    map_matrix.for_each(|(x, y)| {
        commands
            .spawn(SpriteBundle {
                texture: textures.water_tile.clone(),
                transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 1.)),
                ..Default::default()
            })
            .insert(WaterTile);
    });
    map_object.water_top = water_tile_y_position;
}

fn spawn_border(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    camera_query: Query<&Transform, With<MainCamera>>,
    mut map_object: ResMut<MapObject>,
) {
    let camera_transform = camera_query.get_single().unwrap();
    let y_where_border_should_be_generated = camera_transform.translation.y + 800.;

    if map_object.border_top > y_where_border_should_be_generated {
        return;
    }
    let border_tiles = [
        textures.border_tile1.clone(),
        textures.border_tile2.clone(),
        textures.border_tile3.clone(),
        textures.border_tile4.clone(),
    ];
    let mut rng = rand::thread_rng();
    // Left side
    let border_y = map_object.border_top + BORDER_TILE_HEIGHT;
    commands
        .spawn(SpriteBundle {
            texture: border_tiles.choose(&mut rng).unwrap().clone(),
            transform: Transform::from_translation(Vec3::new(
                0. + BORDER_TILE_HEIGHT / 2.,
                border_y,
                1.1,
            ))
            .with_rotation(Quat::from_rotation_z(PI)),
            ..Default::default()
        })
        .insert(BorderTile);

    // Right side
    commands
        .spawn(SpriteBundle {
            texture: border_tiles.choose(&mut rng).unwrap().clone(),
            transform: Transform::from_translation(Vec3::new(
                MAP_WIDTH - BORDER_TILE_HEIGHT / 2.,
                border_y,
                1.1,
            )),
            ..Default::default()
        })
        .insert(BorderTile);

    map_object.border_top = border_y;
}

fn spawn_land(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    camera_query: Query<&Transform, With<MainCamera>>,
    mut map_object: ResMut<MapObject>,
) {
    let camera_transform = camera_query.get_single().unwrap();
    let y_where_land_should_be_generated = camera_transform.translation.y + 800.;

    if map_object.land_top > y_where_land_should_be_generated {
        return;
    }
    let land_y = map_object.land_top + LAND_TILE_SIZE;
    for tile_mid in [
        0. - 3. * LAND_TILE_SIZE / 2.,
        0. - LAND_TILE_SIZE / 2.,
        MAP_WIDTH + LAND_TILE_SIZE / 2.,
        MAP_WIDTH + 3. * LAND_TILE_SIZE / 2.,
    ] {
        commands
            .spawn(SpriteBundle {
                texture: textures.land_tile.clone(),
                transform: Transform::from_translation(Vec3::new(tile_mid, land_y, 1.1)),
                ..Default::default()
            })
            .insert(LandTile)
            .insert(Collidable {
                size: Vec2::new(LAND_TILE_SIZE, LAND_TILE_SIZE),
                damage: 100,
            });
    }
    map_object.land_top = land_y;
}

fn despawn_environment(
    mut commands: Commands,
    entities_q: Query<Entity, Or<(With<LandTile>, With<BorderTile>, With<WaterTile>)>>,
    mut map_object: ResMut<MapObject>,
) {
    for entity in entities_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
    *map_object = MapObject::default();
}
