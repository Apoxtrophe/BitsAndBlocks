use bevy::{ecs::observer::TriggerTargets, prelude::*};
use bevy_rapier3d::prelude::Collider;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum VoxelType {
    Stone,
    Dirt,
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
    position: IVec3,
) {

    let voxel = Voxel {
        voxel_type: VoxelType::Stone,
        position,
        state: false,
    };
    
    if !voxel_map.voxels.contains_key(&position) {
       let entity =   
       commands.spawn((
           Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
           MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 0))),
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
