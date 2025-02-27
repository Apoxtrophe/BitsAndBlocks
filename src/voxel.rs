use std::{collections::HashMap, fs};

use bevy::{prelude::*, render::render_resource::encase::private::RuntimeSizedArray};
use bevy_rapier3d::prelude::Collider;

use crate::{
    config::{TEXTURE_PATH, VOXEL_DEFINITITION_PATH},
    graphics::{create_cable_mesh, create_voxel_mesh},
    helpers::{
        compute_voxel_transform, get_neighboring_coords, voxel_exists,
        NEIGHBOR_DIRECTIONS, VOXEL_COLLIDER_SIZE,
    },
};

#[derive(Component, Debug, Copy, Clone)]
pub struct Voxel {
    pub voxel_id: (usize, usize),
    pub position: IVec3,
    pub direction: usize,
    pub state: bool,
}

#[derive(Bundle)]
pub struct VoxelBundle {
    pub voxel: Voxel,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub transform: Transform,
    pub collider: Collider,
}

#[derive(Clone)]
pub struct VoxelAsset {
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<StandardMaterial>,
    pub voxel_definition: VoxelDefinition,
    pub texture_row: usize, 
}


#[derive(Resource, Clone)]
pub struct VoxelMap {
    pub entity_map: HashMap<IVec3, Entity>, // Entity ids by location
    pub voxel_map: HashMap<IVec3, Voxel>, // Local voxel values by location
    pub voxel_asset_map: HashMap<(usize, usize), VoxelAsset>, // global voxel values by id 
}


#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct VoxelDefinition{
    pub voxel_id: (usize, usize),
    pub name: String,
}

/// Setup responcible for loading voxel assets from voxel_definitions.json, and for initializing entity and voxel maps.
pub fn setup_voxels(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture_handle = asset_server.load(TEXTURE_PATH);
    let file_content = fs::read_to_string(VOXEL_DEFINITITION_PATH)
        .expect("Failed to read file");
    let voxel_defs: Vec<VoxelDefinition> = serde_json::from_str(&file_content)
        .expect("Failed to parse JSON");

    let voxel_asset_map = voxel_defs
        .into_iter()
        .enumerate()
        .map(|(i, voxel_def)| {
            let mesh_handle = meshes.add(create_voxel_mesh(i));
            let material_handle = materials.add(create_voxel_material(texture_handle.clone()));
            let texture_row = i;
            (voxel_def.voxel_id, VoxelAsset {
                mesh_handle,
                material_handle,
                voxel_definition: voxel_def,
                texture_row,
            })
        })
        .collect::<HashMap<_, _>>();
    
    let entity_map = HashMap::new();
    let voxel_map = HashMap::new();
    
    commands.insert_resource(VoxelMap { entity_map, voxel_map, voxel_asset_map });
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
        })
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

/// Checks for neighboring voxels and returns an array of booleans.
/// Used for cable connections, and ignores some types of voxel such as structural blocks or cables that do not match. 
pub fn count_neighbors(voxel: Voxel, voxel_map: &VoxelMap) -> [bool; 6] {
    let mut neighbors: [bool; 6] = [false; 6];

    let positions = get_neighboring_coords(voxel.position);

    for i in 0..positions.len() {
        if voxel_map.entity_map.contains_key(&positions[i]) {
            let Some(neighbor_voxel) = voxel_map.voxel_map.get(&positions[i]) else {
                continue;
            };

            let home_id = voxel.voxel_id;
            let neighbor_id = neighbor_voxel.voxel_id;

            if home_id == neighbor_id || neighbor_id.0 > 1{
                neighbors[i] = true;
            }
            
            if home_id.0 == 2 && neighbor_id.0 == 2 {
                neighbors[i] = true;
            }
        }
    }
    neighbors
}

/// Updates the cable mesh for a given voxel entity based on its neighbor connections.
fn update_voxel_cable_mesh(
    entity: Entity,
    voxel: &Voxel,
    voxel_map: &VoxelMap,
    meshes: &mut Assets<Mesh>,
    commands: &mut Commands,
) {
    let connections = count_neighbors(*voxel, voxel_map);
    let image_row = voxel_map.voxel_asset_map[&voxel.voxel_id].texture_row;
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
        if let Some(entity) = voxel_map.entity_map.get(pos) {
            if let Ok((entity, voxel)) = query.get_mut(*entity) {
                // If the voxel is a cable-type, update its mesh.
                if voxel.voxel_id.0 == 1 || voxel.voxel_id.0 == 2 {
                    update_voxel_cable_mesh(entity, voxel, voxel_map, meshes, commands);
                }
            }
        }
    }
}
