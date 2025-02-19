use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

use crate::{graphics::create_voxel_mesh, VoxelReasources};

#[derive(Debug, Copy, Clone)]
pub enum VoxelType {
    Stone,
}

/// Voxel Logic Component
#[derive(Component, Debug, Copy, Clone)]
pub struct Voxel {
    pub voxel_type: VoxelType,
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
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<StandardMaterial>,
}

#[derive(Resource)]
pub struct VoxelAssets {
    pub texture_atlas: Handle<Image>,
    pub stone_assets: VoxelAsset,

}

pub fn setup_voxel_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_atlas: Handle<Image> = asset_server.load("textures/texturepack.png");
    let stone_assets = VoxelAsset {
        mesh_handle: meshes.add(create_voxel_mesh(0)),
        material_handle: materials.add(create_voxel_material(texture_atlas.clone())),
    };
    
    commands.insert_resource(VoxelAssets {
        texture_atlas,
        stone_assets,
    });
}

/// A resource to track voxel entities by their position.

pub fn add_voxel(
    mut commands: Commands,
    mut voxel_resources: ResMut<VoxelReasources>,
    voxel_assets: Res<VoxelAssets>,
    voxel: Voxel,
) {
    if voxel_resources.voxel_map.contains_key(&voxel.position) {
        return;
    }

    let (mesh_handle, material_handle) = match voxel.voxel_type {
        VoxelType::Stone => (
            voxel_assets.stone_assets.mesh_handle.clone(),
            voxel_assets.stone_assets.material_handle.clone(),
        ),
    };
    
    let transform = Transform {
        translation: voxel.position.as_vec3(),
        rotation: Quat::from_rotation_y(FRAC_PI_2 * voxel.direction as f32 + PI),
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
    mut voxel_map: ResMut<VoxelReasources>,
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