use bevy::prelude::*;
use crate::prelude::*;
use bevy_kira_audio::AudioSource;


#[derive(Event, Debug)]
pub enum AudioEvent {
    Place {},
    Destroy {}, 
    UIHover {}, 
    UIClick {},
}

fn calculate_world_volume (player: &Res<Player>) -> f32 {
    let distance = player.distance;
    let volume = 1.0 / (distance + 1.0);
    volume
}

pub fn audio_event_handler (
    audio: Res<Audio>,
    audio_handles: Res<AudioHandles>,
    mut event_reader: EventReader<AudioEvent>,
    player: Res<Player>,
) {
    for event in event_reader.read() {
        match event {
            AudioEvent::Place {} => {
                let volume = calculate_world_volume(&player);
                audio.play(audio_handles.place.clone()).with_volume(Volume::Amplitude(volume as f64));
            }
            AudioEvent::Destroy {} => {
                let volume = calculate_world_volume(&player);
                audio.play(audio_handles.destroy.clone()).with_volume(Volume::Amplitude(volume as f64));
            }
            AudioEvent::UIHover {} => {
                audio.play(audio_handles.ui_hover.clone()).with_volume(UI_VOLUME);
            }
            AudioEvent::UIClick {} => {
                audio.play(audio_handles.ui_click.clone()).with_volume(UI_VOLUME);
            }
        }
    }
}