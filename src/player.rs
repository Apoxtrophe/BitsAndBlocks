use std::f32::consts::TAU;

use bevy::{
    prelude::*, render::camera::Exposure, window::CursorGrabMode
};
use bevy_rapier3d::prelude::*;

use bevy_fps_controller::controller::*;

use crate::{events::GameEvent, helpers::{cardinalize, get_neighboring_coords}, voxel::{Voxel, VoxelAssets}};

const SPAWN_POINT: Vec3 = Vec3::new(0.0, 5.625, 0.0);

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Resource)]
pub struct PlayerData {
    pub camera_pos: Vec3,
    pub camera_dir: Vec3,
    pub ray_hit_pos: Vec3,
    pub selected: Vec3,
    pub selected_adjacent: Vec3,
    pub selector: usize,
    pub hotbar_ids: Vec<(usize, usize)>,
}

impl Default for PlayerData {
    fn default() -> Self {
        let mut hotbar_ids = Vec::new();
        for i in 0..9 {
            hotbar_ids.push((i, 0));
        }
        Self {
            camera_pos: Vec3::ZERO,
            camera_dir: Vec3::ZERO,
            ray_hit_pos: Vec3::ZERO,
            selected: Vec3::ZERO,
            selected_adjacent: Vec3::ZERO,
            selector: 0,
            hotbar_ids,
        }
    }
}

pub fn setup_player(mut commands: Commands) {

    let height = 3.0;
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
                pitch: -TAU / 12.0,
                yaw: TAU * 5.0 / 8.0,
                ..default()
            },
            FpsController {
                air_acceleration: 80.0,
                ..default()
            },
        ))
        .insert(CameraConfig {
            height_offset: -0.5,
        })
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
}

/// Respawn entities whose vertical position falls below the threshold.
pub fn respawn_system(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in query.iter_mut() {
        // Only respawn if the entity is below -50.0
        if transform.translation.y > -50.0 {
            continue;
        }
        velocity.linvel = Vec3::ZERO;
        transform.translation = SPAWN_POINT;
    }
}

pub fn update_cursor_and_input(
    window: &mut Window,
    controller_query: &mut Query<&mut FpsController>,
    grab_mode: CursorGrabMode,
    cursor_visible: bool,
    input_enabled: bool,
) {
    window.cursor_options.grab_mode = grab_mode;
    window.cursor_options.visible = cursor_visible;
    for mut controller in controller_query.iter_mut() {
        controller.enable_input = input_enabled;
    }
}

pub fn input_event_system(
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Res<PlayerData>,
    voxel_assets: Res<VoxelAssets>,
    mut window_query: Query<&mut Window>,
    mut event_writer: EventWriter<GameEvent>,
) {
    // --- Cursor and Input Mode Updates ---
    if let Ok(_) = window_query.get_single_mut() {
        if mouse_input.just_pressed(MouseButton::Left) {
            event_writer.send(GameEvent::UpdateCursor {
                mode: CursorGrabMode::Locked,
                show_cursor: false,
                enable_input: true,
            });
        } else if keyboard_input.just_pressed(KeyCode::Escape) {
            event_writer.send(GameEvent::UpdateCursor {
                mode: CursorGrabMode::None,
                show_cursor: true,
                enable_input: false,
            });
        } else if keyboard_input.pressed(KeyCode::Tab) {
            event_writer.send(GameEvent::UpdateCursor {
                mode: CursorGrabMode::Locked,
                show_cursor: true,
                enable_input: false,
            });
        } else if keyboard_input.just_released(KeyCode::Tab) {
            event_writer.send(GameEvent::UpdateCursor {
                mode: CursorGrabMode::Locked,
                show_cursor: false,
                enable_input: true,
            });
        }
    }

    // --- Player Action Events ---
    
    let direction = cardinalize(player.camera_dir);
    
    if mouse_input.just_pressed(MouseButton::Left) && !keyboard_input.pressed(KeyCode::Tab) {
        let voxel = Voxel {
            position: player.selected_adjacent.as_ivec3(),
            voxel_id: player.hotbar_ids[player.selector],
            state: false,
            direction,
        };
        let voxel_asset = voxel_assets.voxel_asset_map[&voxel.voxel_id].clone();
        
        event_writer.send(GameEvent::PlaceBlock { voxel, voxel_asset });
        // Meshes that need updating to event handler
        let mesh_updates = get_neighboring_coords(voxel.position);
        event_writer.send(GameEvent::UpdateMeshCall { updates: mesh_updates });
    } else if mouse_input.just_pressed(MouseButton::Right) {
        event_writer.send(GameEvent::RemoveBlock { position: player.selected.as_ivec3() });
        let mesh_updates = get_neighboring_coords(player.selected.as_ivec3());
        event_writer.send(GameEvent::UpdateMeshCall { updates: mesh_updates });
    }
}

