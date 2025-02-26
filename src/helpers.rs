use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use crate::{config::{ROTATION_LOCKED_SUBSETS, SUBSET_SIZES}, voxel::Voxel, VoxelMap};

pub const VOXEL_COLLIDER_SIZE: f32 = 0.5;

pub const NEIGHBOR_DIRECTIONS: [IVec3; 6] = [
    IVec3::new(1, 0, 0),
    IVec3::new(-1, 0, 0),
    IVec3::new(0, 1, 0),
    IVec3::new(0, -1, 0),
    IVec3::new(0, 0, 1),
    IVec3::new(0, 0, -1),
];

// Helper functions should be small in scope and self explanitory. Used only for the readability of code. 
/// Given the id of a voxel, returns the row in the texture atlas.
pub fn texture_row(
    voxel_id: (usize,usize),
) -> usize {
    let offsets: Vec<_> = SUBSET_SIZES
        .iter()
        .scan(0, |state, &size| {
            let offset = *state;
            *state += size;
            Some(offset)
        })
        .collect();
    offsets[voxel_id.0] + voxel_id.1
}

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
    voxel_map.voxel_map.contains_key(&position)
}

/// Calculates the rotation factor for a voxel.
pub fn get_voxel_rotation_factor(voxel: &Voxel) -> f32 {
    if voxel.voxel_id.0 <= ROTATION_LOCKED_SUBSETS { 0.0 } else { 1.0 }
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
