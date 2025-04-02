use bevy::prelude::*;

use crate::prelude::*;


pub fn raycast_system(
    query: Query<&GlobalTransform, With<PlayerCamera>>,
    mut ray_cast: MeshRayCast,
    mut gizmos: Gizmos,
    mut player: ResMut<Player>,
    voxel_map: Res<VoxelMap>,
) {
    // Retrieve the camera transform or exit early.
    let camera_transform = if let Ok(transform) = query.get_single() {
        transform
    } else {
        return;
    };

    // Compute the camera's position and forward direction (-Z axis by default).
    let camera_position = camera_transform.translation();
    let camera_forward = camera_transform.rotation() * Vec3::new(0.0, 0.0, -1.0);

    // Create a ray from the camera's position along its forward vector.
    let ray = Ray3d::new(
        camera_position,
        Dir3::new(camera_forward).expect("Invalid camera forward direction"),
    );

    // Cast the ray and process the first intersection, if any.
    if let Some((_, intersection)) = ray_cast.cast_ray(ray, &RayCastSettings::default()).first() {
        let normal = intersection.normal.round();
        let triangle = intersection.triangle.expect("Missing triangle data");
        let avg = (triangle[0] + triangle[1] + triangle[2]) / 3.0;
        let selected_voxel_pos = (avg - normal * 0.5).round();
        let mut adjacent_voxel_pos = selected_voxel_pos + normal;
        let hit_voxel = voxel_map.voxel_map.get(&selected_voxel_pos.as_ivec3());
        let hit_point = intersection.point;

        let distance = intersection.distance;
        // Adjust adjacent voxel position for ground collisions.
        if hit_point.y < 0.51 {
            adjacent_voxel_pos = (hit_point + Vec3::Y * 0.5).round();
        }

        // Determine if we should select the voxel (within maximum distance).
        let selected_voxel = if intersection.distance <= MAX_RAY_DIST {
            Some(Voxel {
                voxel_id: player.hotbar_ids[player.hotbar_selector],
                position: adjacent_voxel_pos.as_ivec3(),
                direction: 0, // Updated elsewhere.
                state: false,
            })
        } else {
            None
        };

        // Draw a debug cuboid at the adjusted voxel location.
        gizmos.cuboid(Transform::from_translation(adjacent_voxel_pos - normal), Color::BLACK);

        // Update player data with the ray hit details.
        player.ray_hit_pos = hit_point;
        player.hit_voxel = hit_voxel.cloned();
        player.selected_voxel = selected_voxel;
        player.distance = distance;
    }

    // Update the player's selected voxel descriptor using a more concise mapping.
    player.selected_descriptor = voxel_map
        .asset_map
        .get(&player.hotbar_ids[player.hotbar_selector])
        .map(|descriptor| descriptor.clone().definition);

    // Always update the camera position and direction.
    player.camera_pos = camera_position;
    player.camera_dir = camera_forward;
}
