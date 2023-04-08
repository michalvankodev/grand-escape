use bevy::prelude::*;

use crate::actions::game_control::{get_movement, GameControl};
use crate::GameState;

mod game_control;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>()
            .add_system(pause_game.in_set(OnUpdate(GameState::Playing)))
            .add_system(unpause_game.in_set(OnUpdate(GameState::Paused)))
            .add_system(set_movement_actions.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
}

pub fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    let player_movement = Vec2::new(
        get_movement(GameControl::Right, &keyboard_input)
            - get_movement(GameControl::Left, &keyboard_input),
        0.0, // get_movement(GameControl::Up, &keyboard_input)
             //     - get_movement(GameControl::Down, &keyboard_input),
    );

    if player_movement != Vec2::ZERO {
        actions.player_movement = Some(player_movement.normalize());
    } else {
        actions.player_movement = None;
    }
}

fn pause_game(mut state: ResMut<NextState<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        state.set(GameState::Paused);
    }
}

fn unpause_game(mut state: ResMut<NextState<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        state.set(GameState::Playing);
    }
}
