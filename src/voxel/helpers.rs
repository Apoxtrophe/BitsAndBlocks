use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{prelude::*, render::mesh::VertexAttributeValues};

use crate::prelude::*;

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
    match voxel.t {
        VoxelType::Structural(_) => 0.0,
        VoxelType::Wire(_) => 0.0,
        VoxelType::BundledWire => 0.0,
        _ => 1.0,
    }
}

/// Converts a 3D direction vector into one of four cardinal direction indices (1 through 4).
/// Returns 1 by default if the horizontal component is negligible.
pub fn cardinalize(dir: Vec3) -> usize {
    let horizontal = Vec2::new(dir.x, dir.z);
    
    if horizontal.length_squared() < 1e-6 {
        return 1;
    }
    
    // Compute the angle in radians, ensuring it's within [0, 2π).
    let mut angle = horizontal.x.atan2(horizontal.y);
    angle = angle.rem_euclid(2.0 * std::f32::consts::PI);
    
    // Divide the circle into four sectors and round to the nearest sector.
    let sector = (angle / (std::f32::consts::PI / 2.0)).round() as i32 % 4;
    (sector + 1) as usize
}



pub fn tile_mesh_uvs(mesh: &mut Mesh, tiling_factor: f32) {
    if let Some(VertexAttributeValues::Float32x2(uvs)) = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
        for uv in uvs.iter_mut() {
            uv[0] *= tiling_factor;
            uv[1] *= tiling_factor;
        }
    }
}

