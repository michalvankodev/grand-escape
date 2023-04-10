use crate::actions::{set_movement_actions, Actions};
use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_audio_channel::<WaterAudioChannel>()
            .add_system(start_audio.in_schedule(OnEnter(GameState::Playing)))
            .add_system(
                control_water_sound
                    .after(set_movement_actions)
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[derive(Resource)]
struct WaterAudioChannel;

#[derive(Resource)]
struct WaterAudio(Handle<AudioInstance>);

fn start_audio(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    audio: Res<AudioChannel<WaterAudioChannel>>,
) {
    audio.pause();
    let handle = audio
        .play(audio_assets.water.clone())
        .looped()
        .with_volume(0.3)
        .handle();
    commands.insert_resource(WaterAudio(handle));
}

fn control_water_sound(
    actions: Res<Actions>,
    audio: Res<WaterAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&audio.0) {
        match instance.state() {
            PlaybackState::Paused { .. } => {
                if actions.player_movement.is_some() {
                    instance.resume(AudioTween::default());
                }
            }
            PlaybackState::Playing { .. } => {
                if actions.player_movement.is_none() {
                    instance.pause(AudioTween::default());
                }
            }
            _ => {}
        }
    }
}
