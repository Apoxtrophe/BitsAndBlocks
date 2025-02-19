use bevy::prelude::*;

use crate::{config::*, player::{PlayerCamera, PlayerData}};

// Cast a ray from the camera and update player data
pub fn raycast_system(
    query: Query<&GlobalTransform, With<PlayerCamera>>,
    mut ray_cast: MeshRayCast,
    mut gizmos: Gizmos,
    mut player_data: ResMut<PlayerData>,
) {
    // Early return if the camera transform is not found.
    let camera_transform = match query.get_single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    let camera_position = camera_transform.translation();
    // In Bevy the camera faces -Z by default.
    let camera_forward = camera_transform.rotation() * Vec3::new(0.0, 0.0, -1.0);

    let ray = Ray3d::new(
        camera_position,
        Dir3::new(camera_forward).expect("Invalid camera forward direction"),
    );

    if RAY_DEBUG {
        gizmos.line(
            camera_position,
            camera_position + camera_forward * RAY_MAX_DIST,
            Color::BLACK,
        );
    }

    if let Some((_, intersection)) = ray_cast.cast_ray(ray, &RayCastSettings::default()).first() {
        let distance = intersection.distance;
        let normal = intersection.normal.round();
        // We assume the triangle is present.
        let triangle = intersection.triangle.unwrap();
        let avg = (triangle[0] + triangle[1] + triangle[2]) / 3.0;
        let position = (avg - normal * 0.5).round();
        let adjacent_position = position + normal;
        let hit_point = intersection.point;

        gizmos.sphere(hit_point, RAY_SPHERE_RADIUS, Color::BLACK);
        player_data.ray_hit_pos = hit_point;
        player_data.selected = position;
        player_data.selected_adjacent =
            if distance < RAY_MAX_DIST { adjacent_position } else { Vec3::ZERO };
    }

    player_data.camera_pos = camera_position;
    player_data.camera_dir = camera_forward;
}