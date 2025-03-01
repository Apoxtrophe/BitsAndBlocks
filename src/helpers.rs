use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use crate::{config::ROTATION_LOCKED_SUBSETS, voxel::Voxel, VoxelMap};

pub const VOXEL_COLLIDER_SIZE: f32 = 0.5;



// Helper functions should be small in scope and self explanitory. Used only for the readability of code. 

/// Returns all 6 neighbor positions for a given coordinate.
pub fn get_neighboring_coords(coord: IVec3) -> [IVec3; 6] {
    [
        coord + IVec3::new(1, 0, 0),
        coord + IVec3::new(-1, 0, 0),
        coord + IVec3::new(0, 1, 0),
        coord + IVec3::new(0, -1, 0),
        coord + IVec3::new(0, 0, 1),
        coord + IVec3::new(0, 0, -1),
    ]
}

/// Computes the transform for a voxel based on its position and direction.
pub fn compute_voxel_transform(voxel: &Voxel) -> Transform {
    let rotation_factor = get_voxel_rotation_factor(voxel);
    let rotation_angle = rotation_factor * FRAC_PI_2 * voxel.direction as f32 + PI;
    Transform {
        translation: voxel.position.as_vec3(),
        rotation: Quat::from_rotation_y(rotation_angle),
        scale: Vec3::ONE,
    }
}


/// Returns true if a voxel already exists at the given position.
pub fn voxel_exists(voxel_map: &VoxelMap, position: IVec3) -> bool {
    voxel_map.entity_map.contains_key(&position)
}

/// Calculates the rotation factor for a voxel.
pub fn get_voxel_rotation_factor(voxel: &Voxel) -> f32 {
    if voxel.voxel_id.0 <= ROTATION_LOCKED_SUBSETS { 0.0 } else { 1.0 }
}

/// Converts a 3D direction into one of four cardinal directions as an index (1 through 4).
pub fn cardinalize(dir: Vec3) -> usize {
    let horizontal = Vec2::new(dir.x, dir.z);

    if horizontal.length_squared() < 1e-6 {
        return 1;
    }

    let mut angle = horizontal.x.atan2(horizontal.y);
    angle = angle.rem_euclid(2.0 * std::f32::consts::PI);

    let sector = (angle / (std::f32::consts::PI / 2.0)).round() as i32 % 4;

    (sector + 1) as usize
}
