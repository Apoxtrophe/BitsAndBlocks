use std::{
    collections::HashMap,
    f32::consts::{FRAC_PI_2, PI},
};

use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

use crate::{
    config::{OFFSETS, ROTATION_LOCKED_SUBSETS, TEXTURE_PATH, VOXEL_LIST},
    graphics::{create_cable_mesh, create_voxel_mesh}, VoxelMap,
};

#[derive(Resource)]
pub struct VoxelTypes {
    pub voxels: Vec<Vec<(usize, usize)>>,
    pub voxel_texture_row: HashMap<(usize, usize), usize>,
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

/// Helper: Returns true if a voxel already exists at the given position.
fn voxel_exists(voxel_map: &VoxelMap, position: IVec3) -> bool {
    voxel_map.voxel_map.contains_key(&position)
}

/// Helper: Calculates the rotation factor for a voxel (used to lock rotation for specific subsets).
fn get_voxel_rotation_factor(voxel: &Voxel) -> f32 {
    if voxel.voxel_id.0 <= ROTATION_LOCKED_SUBSETS {
        0.0
    } else {
        1.0
    }
}

/// Computes the transform for a voxel based on its position and direction.
fn compute_voxel_transform(voxel: &Voxel) -> Transform {
    let rotation_factor = get_voxel_rotation_factor(voxel);
    let rotation_angle = rotation_factor * FRAC_PI_2 * voxel.direction as f32 + PI;
    Transform {
        translation: voxel.position.as_vec3(),
        rotation: Quat::from_rotation_y(rotation_angle),
        scale: Vec3::ONE,
    }
}

/// Helper: Spawns a voxel entity and returns its Entity id.
fn spawn_voxel_entity(commands: &mut Commands, voxel: Voxel, voxel_asset: &VoxelAsset) -> Entity {
    let transform = compute_voxel_transform(&voxel);
    commands
        .spawn(VoxelBundle {
            voxel,
            mesh: Mesh3d(voxel_asset.mesh_handle.clone()),
            material: MeshMaterial3d(voxel_asset.material_handle.clone()),
            transform,
            collider: Collider::cuboid(0.5, 0.5, 0.5),
        })
        .id()
}

/// Loads the texture atlas and creates voxel assets (meshes and materials) using an iterator.
pub fn setup_voxel_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_atlas: Handle<Image> = asset_server.load(TEXTURE_PATH);

    let voxel_assets = VOXEL_LIST
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

/// Spawns a voxel entity if one is not already present at the specified position.
pub fn add_voxel(
    commands: &mut Commands,
    voxel_resources: &mut VoxelMap,
    voxel_asset: VoxelAsset,
    voxel: Voxel,
) {
    if voxel_exists(voxel_resources, voxel.position) {
        return;
    }
    let entity = spawn_voxel_entity(commands, voxel, &voxel_asset);
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

/// Helper: Updates the cable mesh for a given voxel entity based on its neighbor connections.
fn update_voxel_cable_mesh(
    entity: Entity,
    position: IVec3,
    voxel: &Voxel,
    voxel_map: &VoxelMap,
    meshes: &mut Assets<Mesh>,
    commands: &mut Commands,
) {
    let connections = count_neighbors(position, voxel_map);
    let image_row = OFFSETS[voxel.voxel_id.0] + voxel.voxel_id.1;
    let new_mesh_handle = meshes.add(create_cable_mesh(image_row, connections));
    commands.entity(entity).insert(Mesh3d(new_mesh_handle));
}

/// Updates meshes, especially cables which need to change mesh to connect to those around it.
pub fn update_meshes(
    voxel_positions: [IVec3; 6],
    voxel_map: &VoxelMap,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    query: &mut Query<(Entity, &Voxel)>,
) {
    for pos in voxel_positions.iter() {
        if let Some(entity) = voxel_map.voxel_map.get(pos) {
            if let Ok((entity, voxel)) = query.get_mut(*entity) {
                println!("{:?}", voxel);
                // If the voxel is a cable-type, update its mesh.
                if voxel.voxel_id.0 == 1 || voxel.voxel_id.0 == 2 {
                    update_voxel_cable_mesh(entity, *pos, voxel, voxel_map, meshes, commands);
                }
            }
        }
    }
}
