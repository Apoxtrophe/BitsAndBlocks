use std::f32::consts::TAU;

use bevy::{
    prelude::*, render::camera::Exposure
};
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_rapier3d::prelude::*;

use bevy_fps_controller::controller::*;

use crate::prelude::*;

pub fn setup_player(mut commands: Commands) {

    let height = PLAYER_HEIGHT;
    let logical_entity = commands
        .spawn((
            Collider::cylinder(height / 2.0, 0.5),
            // A capsule can be used but is NOT recommended
            // If you use it, you have to make sure each segment point is
            // equidistant from the translation of the player transform
            // Collider::capsule_y(height / 2.0, 0.5),
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            RigidBody::Dynamic,
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
            AdditionalMassProperties::Mass(1.0),
            GravityScale(0.0),
            Ccd { enabled: true }, // Prevent clipping when going fast
            Transform::from_translation(SPAWN_POINT),
            LogicalPlayer,
            FpsControllerInput {
                pitch: -0. / 12.0,
                yaw: TAU * 5.0 / 8.0,
                ..default()
            },
            FpsController {
                height: PLAYER_HEIGHT,
                upright_height: PLAYER_HEIGHT,
                crouch_height: PLAYER_CROUCHED_HEIGHT,
                air_acceleration: 80.0,
                key_up: KeyCode::Space,
                key_down: KeyCode::ShiftLeft,
                key_sprint: KeyCode::AltLeft,
                ..default()
            },
        ))
        .insert(CameraConfig {
            height_offset: -0.5,
        }).insert(GameEntity)
        .id();

    let mut player_camera = commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: TAU / 5.0,
            ..default()
        }),
        Exposure{
            ev100: 14.0,
            ..default()
        },
        RenderPlayer { logical_entity },
    ));
    // Insert player camera component
    player_camera.insert(PlayerCamera);
    player_camera.insert(AtmosphereCamera::default());
    player_camera.insert(GameEntity);
}