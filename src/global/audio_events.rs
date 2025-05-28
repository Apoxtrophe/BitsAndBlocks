use bevy::prelude::*;
use crate::prelude::*;


#[derive(Event, Debug, Clone, Copy)]
pub enum AudioEvent {
    Ui(HudSfx),
    World(WorldSfx, IVec3 /* source pos */),
}

#[derive(Debug, Clone, Copy)]
pub enum HudSfx { Hover, Click }
#[derive(Debug, Clone, Copy)]
pub enum WorldSfx { Place, Destroy }

pub fn audio_event_handler (
    audio: Res<Audio>,
    handles: Res<AudioHandles>,
    mut event_reader: EventReader<AudioEvent>,
    player: Res<Player>,
) {
    for event in event_reader.read() {
        match event {
            AudioEvent::Ui(kind) => {
                let (h, v) = match kind {
                    HudSfx::Hover  => (handles.ui_hover.clone(), 0.35),
                    HudSfx::Click  => (handles.ui_click.clone(), 0.45),
                };
                audio.play(h).with_volume(Volume::Amplitude(v));
            }
            AudioEvent::World(kind, src) => {
                let dist   = player.camera_pos.distance(src.as_vec3());
                let volume = (1.0 / (dist + 1.0)).clamp(0.05, 1.0);
                let handle = match kind {
                    WorldSfx::Place   => handles.place.clone(),
                    WorldSfx::Destroy => handles.destroy.clone(),
                };
                audio.play(handle).with_volume(Volume::Amplitude(volume as f64));
            }
        }
    }
}