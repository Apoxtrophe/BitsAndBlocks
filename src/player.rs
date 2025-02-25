use std::{cmp::Ordering, f32::consts::TAU};

use bevy::{
    input::mouse::MouseWheel, prelude::*, render::camera::Exposure, window::CursorGrabMode
};
use bevy_rapier3d::prelude::*;

use bevy_fps_controller::controller::*;

use crate::{config::SUBSET_SIZES, events::GameEvent, graphics::create_cable_mesh, raycast::cardinalize, voxel::{add_voxel, count_neighbors, remove_voxel, Voxel, VoxelAssets}, VoxelMap};

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
            hotbar_ids: hotbar_ids,
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

fn update_cursor_and_input(
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

/// Adjusts the window cursor and FPS controller input based on mouse and keyboard events.
pub fn cursor_system(
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    // For a single-window game, you might want to only update the primary window.
    if let Ok(mut window) = window_query.get_single_mut() {
        if mouse_input.just_pressed(MouseButton::Left) {
            update_cursor_and_input(&mut window, &mut controller_query, CursorGrabMode::Locked, false, true);
        } else if keyboard_input.just_pressed(KeyCode::Escape) {
            update_cursor_and_input(&mut window, &mut controller_query, CursorGrabMode::None, true, false);
        } else if keyboard_input.pressed(KeyCode::Tab) {
            // When inventory is open, we might want the cursor visible even if locked.
            update_cursor_and_input(&mut window, &mut controller_query, CursorGrabMode::Locked, true, false);
        } else if keyboard_input.just_released(KeyCode::Tab) {
            // When the inventory is closed, revert to game mode.
            update_cursor_and_input(&mut window, &mut controller_query, CursorGrabMode::Locked, false, true);
        }
    }
}
/// Processes player actions based on mouse clicks and scroll events.
pub fn player_action_system(
    mouse: Res<ButtonInput<MouseButton>>,
    mut evr_scroll: EventReader<MouseWheel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: ResMut<PlayerData>,
    voxel_assets: Res<VoxelAssets>,
    mut event_writer: EventWriter<GameEvent>,
) {
    let direction = cardinalize(player.camera_dir);    
    
    if mouse.just_pressed(MouseButton::Left) && !keyboard.pressed(KeyCode::Tab) {
        let voxel = Voxel {
            position: player.selected_adjacent.as_ivec3(),
            voxel_id: player.hotbar_ids[player.selector],
            state: false,
            direction,
        };
        let voxel_asset = voxel_assets.voxel_assets[&voxel.voxel_id].clone();
        
        event_writer.send(GameEvent::PlaceBlock {voxel, voxel_asset});
        
        //add_voxel(commands, voxel_map, voxel_asset, voxel);
    } else if mouse.just_pressed(MouseButton::Right) {
        event_writer.send(GameEvent::RemoveBlock { position: player.selected.as_ivec3() });
    }
    
    let selector = player.selector.clone();
    
    if keyboard.pressed(KeyCode::AltLeft) {
        for event in evr_scroll.read() {
            let n = SUBSET_SIZES[selector]; // total number of items in this hotbar subset
            match event.y.partial_cmp(&0.0) {
                Some(Ordering::Greater) => {
                    // Subtract one, wrapping around by adding n-1 before taking modulo n
                    player.hotbar_ids[selector].1 = (player.hotbar_ids[selector].1 + n - 1) % n;
                },
                Some(Ordering::Less) => {
                    // Add one and wrap automatically
                    player.hotbar_ids[selector].1 = (player.hotbar_ids[selector].1 + 1) % n;
                },
                _ => (),
            }
        }
    } else {
        for event in evr_scroll.read() {
            match event.y.partial_cmp(&0.0) {
                Some(Ordering::Greater) => {
                    // When subtracting 1, add 8 instead (because (x - 1) mod 9 == (x + 8) mod 9)
                    player.selector = (player.selector + 8) % 9;
                },
                Some(Ordering::Less) => {
                    // Increment and wrap-around automatically
                    player.selector = (player.selector + 1) % 9;
                },
                _ => (),
            }
        }
    }
}
