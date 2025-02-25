use std::{
    collections::HashMap,
    f32::consts::{FRAC_PI_2, PI},
};

use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

use crate::{
    config::{ROTATION_LOCKED_SUBSETS, TEXTURE_MAP, TEXTURE_PATH},
    graphics::create_voxel_mesh, VoxelMap,
};

#[derive(Resource)]
pub struct VoxelTypes {
    pub voxels: Vec<Vec<(usize,usize)>>,
    pub voxel_texture_row: HashMap<(usize,usize), usize>,
}

/// Resource holding the texture atlas and mapping voxel IDs to assets.
#[derive(Resource)]
pub struct VoxelAssets {
    pub voxel_assets: HashMap<(usize, usize), VoxelAsset>,
}

/// Voxel logic component.
#[derive(Component, Debug, Copy, Clone)]
pub struct Voxel {
    pub voxel_id: (usize, usize),
    pub position: IVec3,
    pub direction: usize,
    pub state: bool,
}

/// Bundle for voxel entities.
#[derive(Bundle)]
pub struct VoxelBundle {
    pub voxel: Voxel,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub transform: Transform,
    pub collider: Collider,
}

/// Handles for a single voxel asset.
#[derive(Clone)]
pub struct VoxelAsset {
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<StandardMaterial>,
}

/// Loads the texture atlas and creates voxel assets (meshes and materials) using an iterator.
pub fn setup_voxel_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_atlas: Handle<Image> = asset_server.load(TEXTURE_PATH);

    let voxel_assets = TEXTURE_MAP
        .iter()
        .enumerate()
        .map(|(i, &(x, y))| {
            (
                (x, y),
                VoxelAsset {
                    mesh_handle: meshes.add(create_voxel_mesh(i)),
                    material_handle: materials.add(create_voxel_material(texture_atlas.clone())),
                },
            )
        })
        .collect::<HashMap<_, _>>();

    commands.insert_resource(VoxelAssets { voxel_assets });
}

/// Computes the transform for a voxel based on its position and direction.
/// If the voxel ID is in a "rotation locked" subset, its rotation remains unchanged.
fn compute_voxel_transform(voxel: &Voxel) -> Transform {
    let rotation_factor = if voxel.voxel_id.0 <= ROTATION_LOCKED_SUBSETS {
        0.0
    } else {
        1.0
    };
    let rotation_angle = rotation_factor * FRAC_PI_2 * voxel.direction as f32 + PI;
    Transform {
        translation: voxel.position.as_vec3(),
        rotation: Quat::from_rotation_y(rotation_angle),
        scale: Vec3::ONE,
    }
}


/// Spawns a voxel entity if one is not already present at the specified position.
pub fn add_voxel(
    commands: &mut Commands,
    voxel_resources: &mut VoxelMap,
    voxel_asset: VoxelAsset,
    voxel: Voxel,
) {
    // Avoid adding duplicate voxels at the same position.
    if voxel_resources.voxel_map.contains_key(&voxel.position) {
        return;
    }

    let transform = compute_voxel_transform(&voxel);

    let entity = commands
        .spawn(VoxelBundle {
            voxel,
            mesh: Mesh3d(voxel_asset.mesh_handle.clone()),
            material: MeshMaterial3d(voxel_asset.material_handle.clone()),
            transform,
            collider: Collider::cuboid(0.5, 0.5, 0.5),
        })
        .id();

    voxel_resources.voxel_map.insert(voxel.position, entity);
}

/// Removes the voxel entity at the given position.
pub fn remove_voxel(
    commands: &mut Commands,
    voxel_map: &mut VoxelMap,
    position: IVec3,
) {
    if let Some(entity) = voxel_map.voxel_map.remove(&position) {
        commands.entity(entity).despawn();
    }
}

/// Creates a voxel material using the provided texture atlas.
pub fn create_voxel_material(atlas_handle: Handle<Image>) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(atlas_handle),
        metallic: 0.5,
        ..Default::default()
    }
}

/// Constant list of neighbor directions (in 3D).
const NEIGHBOR_DIRECTIONS: [IVec3; 6] = [
    IVec3::new(1, 0, 0),
    IVec3::new(-1, 0, 0),
    IVec3::new(0, 1, 0),
    IVec3::new(0, -1, 0),
    IVec3::new(0, 0, 1),
    IVec3::new(0, 0, -1),
];

/// Checks for the existence of neighboring voxels and returns an array of booleans.
pub fn count_neighbors(voxel_position: IVec3, voxel_map: &VoxelMap) -> [bool; 6] {
    let mut neighbors = [false; 6];
    for (i, &dir) in NEIGHBOR_DIRECTIONS.iter().enumerate() {
        let neighbor_pos = voxel_position + dir;
        neighbors[i] = voxel_map.voxel_map.contains_key(&neighbor_pos);
    }
    neighbors
}