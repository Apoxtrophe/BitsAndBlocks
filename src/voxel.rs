use bevy::{asset::RenderAssetUsages, ecs::observer::TriggerTargets, prelude::*, render::mesh::{Indices, PrimitiveTopology}};
use bevy_rapier3d::prelude::Collider;
use std::collections::HashMap;

use crate::{graphics::create_voxel_mesh, player::PlayerData};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum VoxelType {
    Stone,
    Dirt,
}

pub struct VoxelAsset {
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<StandardMaterial>,
}

impl VoxelAsset {
    pub fn new(
        voxel_type: VoxelType,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) {
        let mesh = Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0)));
    }
}

#[derive(Component)]
pub struct Voxel {
    pub voxel_type: VoxelType,
    pub position: IVec3,
    pub state: bool,
}

/// A resource to track voxel entities by their position.
#[derive(Resource)]
pub struct VoxelMap {
    pub voxels: HashMap<IVec3, Entity>,
}

impl Default for VoxelMap {
    fn default() -> Self {
        Self {
            voxels: HashMap::new(),
        }
    }
}

pub fn add_voxel_system(
    mut commands: Commands,
    mut voxel_map: ResMut<VoxelMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    position: IVec3,
    direction: Vec3,
) {

    let voxel = Voxel {
        voxel_type: VoxelType::Stone,
        position,
        state: false,
    };
    
    let atlas_handle: Handle<Image> = asset_server.load("textures/test.png");
    
    let mesh_handle: Handle<Mesh> = meshes.add(create_voxel_mesh(0,1, direction));
    
    if !voxel_map.voxels.contains_key(&position) {
       let entity = commands.spawn((
           Mesh3d(mesh_handle),
           MeshMaterial3d(materials.add(StandardMaterial {
               base_color_texture: Some(atlas_handle),
               ..default()
           })),
           Transform::from_translation(position.as_vec3()),
       )).insert(Collider::cuboid(0.5, 0.5, 0.5)).insert(voxel).id();

        voxel_map.voxels.insert(position, entity);
    }
}

pub fn remove_voxel(
    mut commands: Commands,
    mut voxel_map: ResMut<VoxelMap>,
    position: IVec3,
) {
    if let Some(entity) = voxel_map.voxels.remove(&position) {
        commands.entity(entity).despawn();
    }
}

