mod actions;
mod audio;
mod enemy;
mod environment;
mod health;
mod loading;
mod menu;
mod obstacle;
mod pause;
mod player;
mod score;
mod ui;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::enemy::EnemyPlugin;
use crate::environment::EnvironmentPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::obstacle::ObstaclePlugin;
use crate::player::PlayerPlugin;

use bevy::app::{App, AppExit};
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
    Restart,
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
            .add_system(exit_system.in_schedule(OnEnter(GameState::Exit)))
            .add_system(play_after_init.in_schedule(OnEnter(GameState::Init)))
            .add_system(init_after_restart.in_schedule(OnEnter(GameState::Restart)))
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

fn init_after_restart(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Init);
}

fn change_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.icon = CursorIcon::Crosshair;
}

fn change_cursor_back(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.icon = CursorIcon::Default;
}

fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}
