use std::f32::consts::TAU;

use bevy::{
    color::palettes::css::*, prelude::*, render::camera::Exposure, window::CursorGrabMode
};
use bevy_rapier3d::{parry::math::Point, prelude::*, rapier::prelude::Ray};

use bevy_fps_controller::controller::*;

use crate::config::{RAY_MAX_DIST, RAY_SPHERE_RADIUS};

const SPAWN_POINT: Vec3 = Vec3::new(0.0, 5.625, 0.0);

#[derive(Component)]
pub struct PlayerCamera;


pub fn setup_player(mut commands: Commands, window: Query<&mut Window>, assets: Res<AssetServer>) {

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

pub fn respawn(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in &mut query {
        if transform.translation.y > -50.0 {
            continue;
        }

        velocity.linvel = Vec3::ZERO;
        transform.translation = SPAWN_POINT;
    }
}

pub fn manage_cursor(
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

pub fn camera_direction_system(
    query: Query<&GlobalTransform,
    With<PlayerCamera>>,
    mut ray_cast: MeshRayCast,
    mut gizmos: Gizmos,
    ) {
    if let Ok(camera_transform) = query.get_single() {
        // Get the camera's world position
        let camera_position = camera_transform.translation();
        
        // Calculate the forward direction.
        // In Bevy, the camera faces -Z by default.
        let camera_forward = camera_transform.rotation() * Vec3::new(0.0, 0.0, -1.0);

        // You can now use these values for raycasting or other purposes.
        println!("Camera position: {:?}", camera_position);
        println!("Camera forward: {:?}", camera_forward);
        
        let origin_point: Point<f32> = Point::from(camera_position);
        
        let dir: Dir3 = Dir3::new(camera_forward).expect("Cannot even");
        
        let ray = Ray3d::new(camera_position, dir);
        
        let max_distance = RAY_MAX_DIST;
        gizmos.line(camera_position, camera_position + camera_forward * max_distance, Color::BLACK);
        
        if let Some((_entity, hit)) = ray_cast.cast_ray(ray, &RayCastSettings::default()).first() {
            
            gizmos.sphere(hit.point, RAY_SPHERE_RADIUS, Color::BLACK);
            println!("Hit point: {:?}", hit.point);
        }
    }
}