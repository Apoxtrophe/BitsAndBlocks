use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

use crate::{graphics::{create_voxel_mesh}, VoxelReasources};

pub enum VoxelType {
    Stone,
    Dirt,
}

/// Voxel Logic Component
#[derive(Component)]
pub struct Voxel {
    pub voxel_type: VoxelType,
    pub position: IVec3,
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

/// A resource to track voxel entities by their position.

pub fn add_voxel(
    mut commands: Commands,
    mut voxel_resources: ResMut<VoxelReasources>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    position: IVec3,
    voxel_type: VoxelType,
    tile_row: usize,
) {
    if voxel_resources.voxel_map.contains_key(&position) {
        return;
    }

    let mesh_handle = create_voxel_mesh(tile_row);
    let material_handle = create_voxel_material(voxel_resources.texture_atlas.clone());

    let new_voxel = Voxel {
        voxel_type,
        position,
        state: false,
    };

    let entity = commands.spawn(VoxelBundle {
        voxel: new_voxel,
        mesh: Mesh3d(meshes.add(mesh_handle)),
        material: MeshMaterial3d(materials.add(material_handle)),
        transform: Transform::from_translation(position.as_vec3()),
        collider: Collider::cuboid(0.5, 0.5, 0.5),
    })
    .id();

    voxel_resources.voxel_map.insert(position, entity);
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
