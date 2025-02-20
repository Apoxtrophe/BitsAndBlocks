use std::{collections::HashMap, f32::consts::{FRAC_PI_2, FRAC_PI_4, PI}};

use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

use crate::{config::{ROTATION_LOCKED_SUBSETS, TEXTURE_MAP, TEXTURE_PATH}, graphics::create_voxel_mesh, VoxelMap};


/// Voxel Logic Component
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

pub struct VoxelAsset {
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<StandardMaterial>,
}

#[derive(Resource)]
pub struct VoxelAssets {
    pub texture_atlas: Handle<Image>,
    pub voxel_assets: HashMap<(usize, usize), VoxelAsset>,
}


pub fn setup_voxel_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_atlas: Handle<Image> = asset_server.load(TEXTURE_PATH);
    
    let mut voxel_assets_map = HashMap::new();
    
    for i in 0..TEXTURE_MAP.len() {
        let (x, y) = TEXTURE_MAP[i];
        let voxel_asset = VoxelAsset {
            mesh_handle: meshes.add(create_voxel_mesh(i)),
            material_handle: materials.add(create_voxel_material(texture_atlas.clone())),
        };
        voxel_assets_map.insert((x, y), voxel_asset);
    }
    
    commands.insert_resource(VoxelAssets {
        texture_atlas,
        voxel_assets: voxel_assets_map,
    });
}

/// A resource to track voxel entities by their position.

pub fn add_voxel(
    mut commands: Commands,
    mut voxel_resources: ResMut<VoxelMap>,
    voxel_assets: Res<VoxelAssets>,
    voxel: Voxel,
) {
    if voxel_resources.voxel_map.contains_key(&voxel.position) {
        return;
    }

    let mesh_handle = voxel_assets.voxel_assets[&voxel.voxel_id].mesh_handle.clone();
    let material_handle = voxel_assets.voxel_assets[&voxel.voxel_id].material_handle.clone();
    
    let mut rotating = 1.0; // For some block no rotation is applied
    if voxel.voxel_id.0 <=ROTATION_LOCKED_SUBSETS {
        rotating = 0.0;
    }
    let transform = Transform {
        translation: voxel.position.as_vec3(),
        rotation: Quat::from_rotation_y(rotating * FRAC_PI_2 * voxel.direction as f32 + PI),
        scale: Vec3::ONE,
    };
    
    let entity = commands.spawn(VoxelBundle {
        voxel,
        mesh: Mesh3d(mesh_handle),
        material: MeshMaterial3d(material_handle),
        transform,
        collider: Collider::cuboid(0.5, 0.5, 0.5),
    }).id();
    
    
    voxel_resources.voxel_map.insert(voxel.position, entity);
}

pub fn remove_voxel(
    mut commands: Commands,
    mut voxel_map: ResMut<VoxelMap>,
    position: IVec3,
) {
    if let Some(entity) = voxel_map.voxel_map.remove(&position) {
        commands.entity(entity).despawn();
    }
}
/// Create a material using the shared texture atlas.
pub fn create_voxel_material(atlas_handle: Handle<Image>) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(atlas_handle),
        ..Default::default()
    }
}