
use std::{collections::HashMap, fs};

use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

use crate::prelude::*;

#[derive(Bundle)]
pub struct VoxelBundle {
    pub voxel: Voxel,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub transform: Transform,
    pub collider: Collider,
}

/// Spawns a voxel entity and returns its Entity id.
fn spawn_voxel_entity(commands: &mut Commands, voxel: Voxel, asset: &VoxelAsset) -> Entity {
    let transform = compute_voxel_transform(&voxel);
    commands
        .spawn(VoxelBundle {
            voxel,
            mesh: Mesh3d(asset.mesh_handle.clone()),
            material: MeshMaterial3d(asset.material_handle.clone()),
            transform,
            collider: Collider::cuboid(
                VOXEL_COLLIDER_SIZE,
                VOXEL_COLLIDER_SIZE,
                VOXEL_COLLIDER_SIZE,
            ),
        }).insert(GameEntity)
        .id()
}


/// Spawns a voxel entity if one is not already present at the specified position.
pub fn add_voxel(
    commands: &mut Commands,
    voxel_map: &mut VoxelMap,
    asset: VoxelAsset,
    voxel: Voxel,
) {
    if voxel_exists(voxel_map, voxel.position) {
        return;
    }
    let entity = spawn_voxel_entity(commands, voxel, &asset);
    voxel_map.entity_map.insert(voxel.position, entity);
    voxel_map.voxel_map.insert(voxel.position, voxel);
}

/// Removes the voxel entity at the given position.
pub fn remove_voxel(commands: &mut Commands, voxel_map: &mut VoxelMap, position: IVec3) {
    if let Some(entity) = voxel_map.entity_map.remove(&position) {
        voxel_map.voxel_map.remove(&position);
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

pub fn create_voxel_map(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    image_handle: Handle<Image>,
) -> VoxelMap{
    //let texture_handle = asset_server.load(VOXEL_TEXTURE_PATH);
    let file_content = fs::read_to_string(VOXEL_DEFINITITION_PATH)
        .expect("Failed to read file");
    let voxel_defs: Vec<VoxelDefinition> = serde_json::from_str(&file_content)
        .expect("Failed to parse JSON");

    let voxel_asset_map = voxel_defs
        .into_iter()
        .enumerate()
        .map(|(i, voxel_def)| {
            let mesh_handle = meshes.add(create_voxel_mesh(i));
            let material_handle = materials.add(create_voxel_material(image_handle.clone()));
            let texture_row = i;
            (voxel_def.voxel_id, VoxelAsset {
                mesh_handle,
                material_handle,
                definition: voxel_def,
                texture_row,
            })
        })
        .collect::<HashMap<_, _>>();
    
    let entity_map = HashMap::new();
    let voxel_map = HashMap::new();
    
    let voxel_map = VoxelMap {
        entity_map,
        voxel_map,
        asset_map: voxel_asset_map,
    };
    
    voxel_map
}

/// Checks for neighboring voxels and returns an array of booleans.
/// Used for cable connections, and ignores some types of voxel such as structural blocks or cables that do not match. 
pub fn count_neighbors(voxel: Voxel, voxel_map: &VoxelMap) -> [bool; 6] {
    let mut neighbors = [false; 6];
    let neighbor_positions = get_neighboring_coords(voxel.position);

    for (i, pos) in neighbor_positions.iter().enumerate() {
        // Only consider positions that are occupied.
        if voxel_map.entity_map.contains_key(pos) {
            if let Some(neighbor_voxel) = voxel_map.voxel_map.get(pos) {
                let home_id = voxel.voxel_id;
                let neighbor_id = neighbor_voxel.voxel_id;

                // Valid connection if the types match, or the neighbor's id is greater than 1,
                // or in the special case where home is type 2 and neighbor is type 1.
                if home_id == neighbor_id || neighbor_id.0 > 1 || (home_id.0 == 2 && neighbor_id.0 == 1) {
                    neighbors[i] = true;
                }
            }
        }
    }
    neighbors
}

/// Updates the cable mesh for a given voxel entity based on its neighbor connections.
pub fn update_voxel_cable_mesh(
    entity: Entity,
    voxel: &Voxel,
    voxel_map: &VoxelMap,
    meshes: &mut Assets<Mesh>,
    commands: &mut Commands,
) {
    let connections = count_neighbors(*voxel, voxel_map);
    let texture_row = voxel_map.asset_map[&voxel.voxel_id].texture_row;
    let new_mesh_handle = meshes.add(create_cable_mesh(texture_row, connections));
    commands.entity(entity).insert(Mesh3d(new_mesh_handle));
}

/// Updates meshes, especially cables which need to change mesh to connect to those around them.
pub fn update_meshes(
    voxel_positions: [IVec3; 6],
    voxel_map: &VoxelMap,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    query: &mut Query<(Entity, &Voxel)>,
) {
    for pos in &voxel_positions {
        if let Some(entity) = voxel_map.entity_map.get(pos) {
            if let Ok((entity, voxel)) = query.get_mut(*entity) {
                // Use the helper function for cable voxel checks.
                if is_cable_voxel(voxel) {
                    update_voxel_cable_mesh(entity, voxel, voxel_map, meshes, commands);
                }
            }
        }
    }
}

// Define named constants for cable voxel types.
const CABLE_TYPE_IDS: [usize; 2] = [1, 2];

/// Returns true if the voxel is one of the cable types.
pub fn is_cable_voxel(voxel: &Voxel) -> bool {
    CABLE_TYPE_IDS.contains(&voxel.voxel_id.0)
}
