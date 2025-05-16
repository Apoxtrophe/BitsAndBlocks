use std::{collections::HashMap, fs};

use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

use crate::prelude::*;

#[derive(Bundle)]
pub struct VoxelBundle {
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub transform: Transform,
    pub collider: Collider,
}

fn spawn_voxel_entity(
    commands: &mut Commands,
    voxel: Voxel,
    asset: &VoxelAsset,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {
    let transform = compute_voxel_transform(&voxel);

    // Isolate the borrow from `materials.get` so it ends before calling `materials.add`.
    let new_material_handle = {
        // Get an immutable reference to the template material.
        let template_material = materials
            .get(&asset.material_handle)
            .expect("Failed to get template material");
        // Clone it.
        let cloned_material = template_material.clone();
        // Now add the cloned material to get a new handle.
        materials.add(cloned_material)
    };

    commands
        .spawn(VoxelBundle {
            mesh: Mesh3d(asset.mesh_handle.clone()),
            material: MeshMaterial3d(new_material_handle),
            transform,
            collider: Collider::cuboid(
                VOXEL_COLLIDER_SIZE,
                VOXEL_COLLIDER_SIZE,
                VOXEL_COLLIDER_SIZE,
            ),
        })
        .insert(GameEntity)
        .insert(voxel)
        .id()
}

pub fn add_voxel(
    commands: &mut Commands,
    voxel_map: &mut VoxelMap,
    asset: VoxelAsset,
    voxel: Voxel,
    materials: &mut Assets<StandardMaterial>,
) {
    if voxel_exists(voxel_map, voxel.position) {
        return;
    }
    let entity = spawn_voxel_entity(commands, voxel, &asset, materials);
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

#[inline]
fn cables_compatible(a: &VoxelType, b: &VoxelType) -> bool {
    use VoxelType::*;
    match (a, b) {
        // ─────────────────────────────────────────────────────── wire ↔ wire
        (Wire(ch1), Wire(ch2))           => ch1 == ch2,     // same channel only
        // ─────────────────────────────────────────────── wire ↔ bundled‑wire
        (Wire(_), BundledWire)
      | (BundledWire, Wire(_))
      | (BundledWire, BundledWire)       => true,
        // everything else is “not cable‑cable”
        _                                 => false,
    }
}

/// [+X, −X, +Y, −Y, +Z, −Z]
pub fn count_neighbors(voxel: Voxel, voxel_map: &VoxelMap) -> [bool; 6] {
    let mut neighbors = [false; 6];

    for (i, pos) in get_neighboring_coords(voxel.position).iter().enumerate() {
        // Skip empty space
        let Some(neigh_voxel) = voxel_map.voxel_map.get(pos) else { continue };

        // ── 1. direct cable‑to‑cable decision ───────────────────────────────
        if matches!(voxel.kind, VoxelType::Wire(_) | VoxelType::BundledWire)
        && matches!(neigh_voxel.kind, VoxelType::Wire(_) | VoxelType::BundledWire)
        {
            if cables_compatible(&voxel.kind, &neigh_voxel.kind) {
                neighbors[i] = true;      // compatible channels or bundled
            }
            continue;                     // **never fall through** for cables
        }

        // ── 2. Gate ↔ Cable / Gate ↔ Gate  (I/O aware) ─────────────────────
        let (inputs, output) = voxel_directions(neigh_voxel);
        if inputs.contains(&voxel.position) || output == voxel.position {
            neighbors[i] = true;
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
    let texture_row = voxel_map.asset_map[&voxel.kind].texture_row;
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
                match voxel.kind {
                    VoxelType::BundledWire => {
                        update_voxel_cable_mesh(entity, voxel, voxel_map, meshes, commands);
                    }
                    VoxelType::Wire(_) => {
                        update_voxel_cable_mesh(entity, voxel, voxel_map, meshes, commands);
                    }
                    _ => {}
                }
            }
        }
    }
}