use std::{
    collections::HashMap,
    f32::consts::{FRAC_PI_2, PI},
};

use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

use crate::{
    config::{ROTATION_LOCKED_SUBSETS, TEXTURE_MAP, TEXTURE_PATH},
    graphics::create_voxel_mesh,
    VoxelMap,
};

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
pub struct VoxelAsset {
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<StandardMaterial>,
}

/// Resource holding the texture atlas and mapping voxel IDs to assets.
#[derive(Resource)]
pub struct VoxelAssets {
    pub voxel_assets: HashMap<(usize, usize), VoxelAsset>,
}

/// Setup voxel assets by loading the texture atlas and creating meshes and materials.
pub fn setup_voxel_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Load the shared texture atlas.
    let texture_atlas: Handle<Image> = asset_server.load(TEXTURE_PATH);
    let mut voxel_assets_map = HashMap::new();

    // Use iterators for better readability.
    for (i, &(x, y)) in TEXTURE_MAP.iter().enumerate() {
        let voxel_asset = VoxelAsset {
            mesh_handle: meshes.add(create_voxel_mesh(i)),
            material_handle: materials.add(create_voxel_material(texture_atlas.clone())),
        };
        voxel_assets_map.insert((x, y), voxel_asset);
    }

    commands.insert_resource(VoxelAssets {
        voxel_assets: voxel_assets_map,
    });
}

/// Adds a voxel entity to the world if one doesn't already exist at the given position.
pub fn add_voxel(
    mut commands: Commands,
    mut voxel_resources: ResMut<VoxelMap>,
    voxel_assets: Res<VoxelAssets>,
    voxel: Voxel,
) {
    // Prevent duplicate voxels at the same position.
    if voxel_resources.voxel_map.contains_key(&voxel.position) {
        return;
    }

    // Retrieve the corresponding asset for this voxel.
    let voxel_asset = &voxel_assets.voxel_assets[&voxel.voxel_id];
    let mesh_handle = voxel_asset.mesh_handle.clone();
    let material_handle = voxel_asset.material_handle.clone();

    // Determine if the voxel should be rotated.
    let rotation_factor = if voxel.voxel_id.0 <= ROTATION_LOCKED_SUBSETS {
        0.0
    } else {
        1.0
    };

    let transform = Transform {
        translation: voxel.position.as_vec3(),
        // Compute rotation based on direction and a fixed offset.
        rotation: Quat::from_rotation_y(rotation_factor * FRAC_PI_2 * voxel.direction as f32 + PI),
        scale: Vec3::ONE,
    };

    // Spawn the voxel entity with its components.
    let entity = commands
        .spawn(VoxelBundle {
            voxel,
            mesh: Mesh3d(mesh_handle),
            material: MeshMaterial3d(material_handle),
            transform,
            collider: Collider::cuboid(0.5, 0.5, 0.5),
        })
        .id();

    voxel_resources.voxel_map.insert(voxel.position, entity);
}

/// Removes a voxel entity from the world based on its position.
pub fn remove_voxel(
    mut commands: Commands,
    mut voxel_map: ResMut<VoxelMap>,
    position: IVec3,
) {
    if let Some(entity) = voxel_map.voxel_map.remove(&position) {
        commands.entity(entity).despawn();
    }
}

/// Creates a material using the shared texture atlas.
pub fn create_voxel_material(atlas_handle: Handle<Image>) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(atlas_handle),
        metallic: 0.5,
        ..Default::default()
    }
}
