use bevy::prelude::*;

use crate::prelude::*;

// This directly modifies the player!
pub fn raycast_system(
    query: Query<&GlobalTransform, With<PlayerCamera>>,
    mut raycast: MeshRayCast,
    mut gizmos: Gizmos,
    mut player: ResMut<Player>,
    voxel_map: Res<VoxelMap>,
) {
    // 1. Fetch camera transform or early‑out.
    let camera_tf = if let Ok(tf) = query.get_single() { tf } else { return };

    // 2. Build the ray.
    let cam_pos   = camera_tf.translation();
    let cam_fwd   = camera_tf.rotation() * Vec3::new(0.0, 0.0, -1.0);
    let ray       = Ray3d::new(cam_pos, Dir3::new(cam_fwd).expect("bad dir"));

    // 3. Cast & process first hit.
    if let Some((_, hit)) = raycast.cast_ray(ray, &RayCastSettings::default()).first() {
        let normal     = hit.normal.round();
        let tri_avg    = (hit.triangle.unwrap()[0] + hit.triangle.unwrap()[1] + hit.triangle.unwrap()[2]) / 3.0;
        let sel_pos    = (tri_avg - normal * 0.5).round();
        let mut adj_pos= sel_pos + normal;
        let hit_voxel  = voxel_map.voxel_map.get(&sel_pos.as_ivec3());
        let distance   = hit.distance;

        // ground tweak
        if hit.point.y < 0.51 {
            adj_pos = (hit.point + Vec3::Y * 0.5).round();
        }
        
        // 3‑a. Build *potential* placement voxel.
        let sel_voxel = if distance <= MAX_RAY_DIST {
            Some(Voxel {
                kind: player.hotbar[player.hotbar_selector],   // ← enum
                position: adj_pos.as_ivec3(),
                direction: 0,                                 // set elsewhere
                state: Bits16::all_zeros(),
            })
        } else { None };
        
        // 3‑b. Debug gizmo.
        gizmos.cuboid(
            Transform::from_translation(adj_pos - normal),
            Color::BLACK,
        );

        // 3‑c. Write back to Player resource.
        player.ray_hit_pos     = hit.point;
        player.hit_voxel       = hit_voxel.cloned();
        player.selected_voxel  = sel_voxel;
        player.distance        = distance;
    }


    // 4. Update descriptor based on the *enum* hot‑bar entry.
    player.selected_descriptor = voxel_map
        .asset_map
        .get(&player.hotbar[player.hotbar_selector])   // ← enum key
        .map(|asset| asset.definition.clone());

    // 5. Keep camera pose in the player resource.
    player.camera_pos = cam_pos;
    player.camera_dir = cam_fwd;
}
