use std::{cmp::Ordering, f32::consts::TAU};

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*, render::camera::Exposure, window::CursorGrabMode
};
use bevy_rapier3d::prelude::*;

use bevy_fps_controller::controller::*;

use crate::{config::SUBSET_SIZES, raycast::cardinalize, voxel::{add_voxel, remove_voxel, Voxel, VoxelAssets}, VoxelReasources};

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
        Exposure::SUNLIGHT,
        RenderPlayer { logical_entity },
    ));
    // Insert player camera component
    player_camera.insert(PlayerCamera);
    
    
    
    
    // Spawn cursor
    commands.spawn((
        Node {
            width: Val::Percent(1.0),
            height: Val::Percent(1.0),
            left: Val::Percent(49.5),
            top: Val::Percent(49.5),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::BLACK),
    ));
}

pub fn respawn_system(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in &mut query {
        if transform.translation.y > -50.0 {
            continue;
        }

        velocity.linvel = Vec3::ZERO;
        transform.translation = SPAWN_POINT;
    }
}

pub fn cursor_system(
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    for mut window in &mut window_query {
        if btn.just_pressed(MouseButton::Left) {
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.cursor_options.visible = false;
            for mut controller in &mut controller_query {
                controller.enable_input = true;
            }
        }
        if key.just_pressed(KeyCode::Escape) {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
            for mut controller in &mut controller_query {
                controller.enable_input = false;
            }
        }
    }
}

pub fn player_action_system(
    mouse: Res<ButtonInput<MouseButton>>,
    mut evr_scroll: EventReader<MouseWheel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: ResMut<PlayerData>,
    commands: Commands,
    voxel_map: ResMut<VoxelReasources>,
    voxel_assets: Res<VoxelAssets>,
) {
    let dir = cardinalize(player.camera_dir);    
    
    if mouse.just_released(MouseButton::Left) {
        let voxel = Voxel {
            position: player.selected_adjacent.as_ivec3(),
            voxel_id: player.hotbar_ids[player.selector],
            state: false,
            direction: dir,
        };
        add_voxel(commands, voxel_map, voxel_assets, voxel);
    }
    else if mouse.just_released(MouseButton::Right) {
        remove_voxel(commands, voxel_map, player.selected.as_ivec3());
    }
    
    let selector = player.selector.clone();
    
    if keyboard.pressed(KeyCode::AltLeft) {
        for event in evr_scroll.read() {
            match event.y.partial_cmp(&0.0) {
                Some(Ordering::Less) => player.hotbar_ids[selector].1 -= 1,
                Some(Ordering::Greater) => player.hotbar_ids[selector].1 += 1,
                _ => (),
            }
            player.hotbar_ids[selector].1 = player.hotbar_ids[selector].1.clamp(0, SUBSET_SIZES[selector] - 1);
        }
    } else {
        for event in evr_scroll.read() {
            match event.y.partial_cmp(&0.0) {
                Some(Ordering::Less) => player.selector -= 1,
                Some(Ordering::Greater) => player.selector += 1,
                _ => (),
            }
            player.selector = player.selector.clamp(0, 8);
        }
    }
    
}
