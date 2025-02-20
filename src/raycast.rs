use bevy::prelude::*;

use crate::{config::*, player::{PlayerCamera, PlayerData}};

pub fn raycast_system(
    query: Query<&GlobalTransform, With<PlayerCamera>>,
    mut ray_cast: MeshRayCast,
    mut gizmos: Gizmos,
    mut player_data: ResMut<PlayerData>,
) {
    // Retrieve the camera's transform; exit early if not found.
    let camera_transform = if let Ok(transform) = query.get_single() {
        transform
    } else {
        return;
    };

    // Extract camera position and compute its forward vector (default facing -Z).
    let camera_position = camera_transform.translation();
    let camera_forward = camera_transform.rotation() * Vec3::new(0.0, 0.0, -1.0);

    // Construct a ray from the camera's position in its forward direction.
    let ray = Ray3d::new(
        camera_position,
        Dir3::new(camera_forward).expect("Invalid camera forward direction"),
    );

    // Draw the debug ray if enabled.
    if RAY_DEBUG {
        gizmos.line(
            camera_position,
            camera_position + camera_forward * RAY_MAX_DIST,
            Color::BLACK,
        );
    }

    // Cast the ray using default settings and process the first intersection, if any.
    if let Some((_, intersection)) = ray_cast.cast_ray(ray, &RayCastSettings::default()).first() {
        let distance = intersection.distance;
        // Round the normal to simplify voxel alignment.
        let normal = intersection.normal.round();

        // Compute the average position of the hit triangle's vertices.
        let triangle = intersection.triangle.expect("Missing triangle data");
        let avg = (triangle[0] + triangle[1] + triangle[2]) / 3.0;

        // Determine the voxel that was hit and its adjacent neighbor.
        let selected_voxel = (avg - normal * 0.5).round();
        let mut adjacent_voxel = selected_voxel + normal;

        let hit_point = intersection.point;
        // Adjust the adjacent voxel if the hit point is low.
        if hit_point.y < 0.6 {
            adjacent_voxel = (hit_point + Vec3::Y * 0.5).round();
        }

        // Draw a debug sphere at the adjusted adjacent voxel location.
        gizmos.sphere(adjacent_voxel - normal * 0.5, RAY_SPHERE_RADIUS, Color::BLACK);

        // Update the player's data with ray hit information.
        player_data.ray_hit_pos = hit_point;
        player_data.selected = selected_voxel;
        player_data.selected_adjacent = if distance < RAY_MAX_DIST {
            adjacent_voxel
        } else {
            Vec3::ZERO
        };
    }

    // Always update the player's camera position and forward direction.
    player_data.camera_pos = camera_position;
    player_data.camera_dir = camera_forward;
}

/// Converts a 3D direction into one of four cardinal directions as an index (1 through 4).
pub fn cardinalize(dir: Vec3) -> usize {
    // Project the direction onto the XZ plane.
    let horizontal = Vec2::new(dir.x, dir.z);

    // If the horizontal component is negligible, default to 1.
    if horizontal.length_squared() < 1e-6 {
        return 1;
    }

    // Calculate the angle (in radians) from the positive Y-axis.
    let mut angle = horizontal.x.atan2(horizontal.y);
    // Normalize the angle to be within [0, 2π).
    angle = angle.rem_euclid(2.0 * std::f32::consts::PI);

    // Divide the circle into four sectors (each π/2 radians) and round to the nearest sector.
    let sector = (angle / (std::f32::consts::PI / 2.0)).round() as i32 % 4;

    // Return a 1-indexed cardinal direction.
    (sector + 1) as usize
}
