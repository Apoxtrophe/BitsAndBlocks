use bevy::prelude::*;

use crate::{config::*, player::{PlayerCamera, PlayerData}};

pub fn raycast_system(
    query: Query<&GlobalTransform, With<PlayerCamera>>,
    mut ray_cast: MeshRayCast,
    mut gizmos: Gizmos,
    mut player_data: ResMut<PlayerData>,
) {
    // Attempt to retrieve the camera's transform; exit early if not found.
    let camera_transform = match query.get_single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    // Determine the camera's position and its forward direction.
    let camera_position = camera_transform.translation();
    // In Bevy, the camera faces the negative Z-axis by default.
    let camera_forward = camera_transform.rotation() * Vec3::new(0.0, 0.0, -1.0);

    // Build a ray starting at the camera's position pointing in its forward direction.
    let ray = Ray3d::new(
        camera_position,
        Dir3::new(camera_forward).expect("Invalid camera forward direction"),
    );

    // Draw a debug line for the ray if debugging is enabled.
    if RAY_DEBUG {
        gizmos.line(
            camera_position,
            camera_position + camera_forward * RAY_MAX_DIST,
            Color::BLACK,
        );
    }

    // Cast the ray and process the first intersection, if any.
    if let Some((_, intersection)) = ray_cast.cast_ray(ray, &RayCastSettings::default()).first() {
        let distance = intersection.distance;
        // Round the normal for easier voxel alignment.
        let normal = intersection.normal.round();

        // Retrieve and compute the average position of the hit triangle's vertices.
        // This assumes that the triangle data is always available.
        let triangle = intersection.triangle.expect("Missing triangle data");
        let avg = (triangle[0] + triangle[1] + triangle[2]) / 3.0;

        // Calculate the voxel position that was hit by offsetting the average by half a unit along the normal.
        let selected_voxel = (avg - normal * 0.5).round();
        // Determine the adjacent voxel position in the direction of the normal.
        let mut adjacent_voxel = selected_voxel + normal;

        // Record the exact hit point.
        let hit_point = intersection.point;

        if intersection.point.y < 0.6 {
            adjacent_voxel = (hit_point + (Vec3::Y * 0.5)).round()
        }
        
        // Draw a debug sphere at the hit point.
        gizmos.sphere(hit_point, RAY_SPHERE_RADIUS, Color::BLACK);

        // Update player data with the hit information.
        player_data.ray_hit_pos = hit_point;
        player_data.selected = selected_voxel;
        // Only set the adjacent selection if within the maximum distance.
        player_data.selected_adjacent = if distance < RAY_MAX_DIST {
            adjacent_voxel
        } else {
            Vec3::ZERO
        };
    }

    // Always update the camera's position and direction in the player data.
    player_data.camera_pos = camera_position;
    player_data.camera_dir = camera_forward;
}

pub fn cardinalize(dir: Vec3) -> usize {
    let horizontal = Vec2::new(dir.x, dir.z);
    
    if horizontal.length_squared() < 1e-6 {
        return 1;
    }
    
    let angle = horizontal.x.atan2(horizontal.y);
    
    let angle = if angle < 0.0 {
        angle + 2.0 * std::f32::consts::PI
    } else {
        angle
    };
    
    let sector = (angle / (std::f32::consts::PI / 2.0)).round() as i32 % 4;
    
    (sector + 1) as usize
}