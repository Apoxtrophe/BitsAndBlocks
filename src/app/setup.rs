use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSamplerDescriptor};
use bevy_atmosphere::plugin::AtmospherePlugin;
use bevy_fps_controller::controller::FpsControllerPlugin;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};
use bevy_simple_text_input::TextInputPlugin;

use crate::prelude::*;

pub fn configure(app: &mut App) {
    configure_resources(app);
    configure_events(app);
    configure_plugins(app);
    configure_global_systems(app);
    configure_states(app);
}

fn configure_resources(app: &mut App) {
    app.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: AMBIENT_LIGHT,
    });
    app.insert_resource(Player::default());
}

fn configure_events(app: &mut App) {
    app.add_event::<GameEvent>();
    app.add_event::<AudioEvent>();
    app.add_event::<LogicEvent>();
}

fn configure_plugins(app: &mut App) {
    app.add_plugins(DefaultPlugins.set(ImagePlugin {
        default_sampler: ImageSamplerDescriptor {
            address_mode_u: ImageAddressMode::Repeat,
            address_mode_v: ImageAddressMode::Repeat,
            address_mode_w: ImageAddressMode::Repeat,
            mag_filter: ImageFilterMode::Nearest,
            min_filter: ImageFilterMode::Linear,
            mipmap_filter: ImageFilterMode::Linear,
            lod_min_clamp: 0.0,
            lod_max_clamp: 0.01,
            ..default()
        },
    }));
    app.add_plugins(AudioPlugin);
    app.add_plugins(AtmospherePlugin);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_plugins(TextInputPlugin);
    //app.add_plugins(RapierDebugRenderPlugin::default());
    app.add_plugins(FpsControllerPlugin);
}

fn configure_global_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_ui_visibility,
            menu_button_system,
            event_handler,
            audio_event_handler,
        ),
    );
}

fn configure_states(app: &mut App) {
    app.init_state::<GameState>();
}