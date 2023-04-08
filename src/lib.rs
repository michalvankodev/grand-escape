mod actions;
mod audio;
mod loading;
mod menu;
mod player;
mod enemy;
mod environment;
mod obstacle;
mod health;
mod ui;
mod score;
mod pause;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::enemy::EnemyPlugin;
use crate::environment::EnvironmentPlugin;
use crate::obstacle::ObstaclePlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use health::HealthPlugin;
use pause::PausePlugin;
use score::ScorePlugin;
use ui::UiPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    Init,
    // During this State the actual game logic is executed
    Playing,
    Paused,
    End,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    Exit,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(HealthPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(EnvironmentPlugin)
            .add_plugin(ObstaclePlugin)
            .add_plugin(UiPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(PausePlugin)
            .add_system(play_after_init.in_schedule(OnEnter(GameState::Init)))
            .add_system(change_cursor.in_schedule(OnEnter(GameState::Playing)))
            .add_system(change_cursor_back.in_schedule(OnExit(GameState::Playing)));

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}

fn play_after_init(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Playing);
}
 
fn change_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.icon = CursorIcon::Crosshair;
}
fn change_cursor_back(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.icon = CursorIcon::Default;
}
