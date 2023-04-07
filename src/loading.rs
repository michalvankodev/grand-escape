use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        )
        .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
    #[asset(path = "fonts/FiraSans-Regular.ttf")]
    pub fira_sans_reg: Handle<Font>,
    #[asset(path = "fonts/FiraMono-Regular.ttf")]
    pub fira_mono: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
    #[asset(path = "textures/boat2-36x64.png")]
    pub boat: Handle<Image>,
    #[asset(path = "textures/boat2-crashed-64x64.png")]
    pub boat_crashed: Handle<Image>,
    #[asset(path = "textures/boat-cannon2-14x24.png")]
    pub boat_cannon: Handle<Image>,
    #[asset(path = "textures/enemy-cannon-64x64.png")]
    pub enemy_cannon: Handle<Image>,
    #[asset(path = "textures/enemy-cannon-crashed-64x64.png")]
    pub enemy_cannon_crashed: Handle<Image>,
    #[asset(path = "textures/bullet-32x32.png")]
    pub bullet: Handle<Image>,
    #[asset(path = "textures/water8-64x64.png")]
    pub water_tile: Handle<Image>,
    #[asset(path = "textures/border1-64x64.png")]
    pub border_tile1: Handle<Image>,
    #[asset(path = "textures/border2-64x64.png")]
    pub border_tile2: Handle<Image>,
    #[asset(path = "textures/border3-64x64.png")]
    pub border_tile3: Handle<Image>,
    #[asset(path = "textures/border4-64x64.png")]
    pub border_tile4: Handle<Image>,
    #[asset(path = "textures/sand3-64x64.png")]
    pub land_tile: Handle<Image>,
    #[asset(path = "textures/obstacle-rock1-58x59.png")]
    pub obstacle_rock1: Handle<Image>,
    #[asset(path = "textures/obstacle-rock2-51x53.png")]
    pub obstacle_rock2: Handle<Image>,
    #[asset(path = "textures/obstacle-rock3-60x41.png")]
    pub obstacle_rock3: Handle<Image>,
    #[asset(path = "textures/logo.png")]
    pub logo: Handle<Image>,
    #[asset(path = "textures/play-button.png")]
    pub btn_play: Handle<Image>,
    #[asset(path = "textures/exit-button.png")]
    pub btn_exit: Handle<Image>,
}
