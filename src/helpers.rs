use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{prelude::*, render::mesh::VertexAttributeValues};

use crate::{config::{HOTBAR_BORDER_COLOR, ROTATION_LOCKED_SETS}, voxel::Voxel, VoxelMap};

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
    if voxel.voxel_id.0 <= ROTATION_LOCKED_SETS { 0.0 } else { 1.0 }
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

pub fn tile_mesh_uvs(mesh: &mut Mesh, tiling_factor: f32) {
    if let Some(VertexAttributeValues::Float32x2(uvs)) = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
        for uv in uvs.iter_mut() {
            uv[0] *= tiling_factor;
            uv[1] *= tiling_factor;
        }
    }
}

/// Creates the hotbar slot bundle with shadow and border.
pub fn box_shadow_node_bundle(
    size: Vec2,
    offset: Vec2,
    spread: f32,
    blur: f32,
    border_radius: BorderRadius,
) -> impl Bundle {
    (   
        Node {
            top: Val::Percent(90.0),
            width: Val::Px(size.x),
            height: Val::Px(size.y),
            border: UiRect::all(Val::Px(6.0)),
            ..default()
        },
        BorderColor(HOTBAR_BORDER_COLOR.into()),
        border_radius,
        BackgroundColor(Color::linear_rgba(0.2, 0.2, 0.2, 0.1)),
        BoxShadow {
            color: Color::BLACK.with_alpha(0.8),
            x_offset: Val::Percent(offset.x),
            y_offset: Val::Percent(offset.y),
            spread_radius: Val::Percent(spread),
            blur_radius: Val::Px(blur),
        },
    )
}