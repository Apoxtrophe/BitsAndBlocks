use std::f32::consts::TAU;

use bevy::{color::palettes::css::*, 
    prelude::*};
use bevy_rapier3d::prelude::*;

use bevy_fps_controller::controller::*;

use crate::config::{WORLD_HEIGHT, WORLD_TRANSFORM, WORLD_WIDTH};


pub fn setup(
    mut commands: Commands, mut window: Query<&mut Window>, assets: Res<AssetServer>,     mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut window = window.single_mut();
    window.title = String::from("Bits&Blocks");
    
    // Spawn Sun 
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 7.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    
    // Spawn World
    // 
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(WORLD_WIDTH, WORLD_HEIGHT, WORLD_WIDTH))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        WORLD_TRANSFORM,
    )).insert(Collider::cuboid(WORLD_WIDTH * 0.5, WORLD_HEIGHT *  0.5, WORLD_WIDTH * 0.5));
}