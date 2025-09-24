use bevy::pbr::{light_consts, MeshMaterial3d};
use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

use crate::prelude::*;

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    image_handles: Res<GameTextures>,
) {
    // Spawn a directional light (Sun)
    commands
        .spawn((
            DirectionalLight {
                illuminance: light_consts::lux::FULL_DAYLIGHT,
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(4.0, 7.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .insert(GameEntity);

    // Create world mesh
    let mut mesh: Mesh = (Cuboid::new(WORLD_WIDTH, 1.0, WORLD_WIDTH)).into();
    let tiling_factor = WORLD_WIDTH;
    tile_mesh_uvs(&mut mesh, tiling_factor);
    let mesh_handle = meshes.add(mesh);

    // Create material with texture
    let image_handle = image_handles.ground_texture.clone();
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        perceptual_roughness: 0.2,
        metallic: 0.2,
        ..default()
    });

    // Spawn the world entity with collider
    commands
        .spawn((
            Mesh3d(mesh_handle),
            MeshMaterial3d(material),
            WORLD_TRANSFORM,
        ))
        .insert(Collider::cuboid(WORLD_WIDTH * 0.5, 0.5, WORLD_WIDTH * 0.5))
        .insert(GameEntity);
}