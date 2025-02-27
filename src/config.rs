use bevy::prelude::*;

// WORLD
pub const WORLD_WIDTH: f32 = 100.0;
pub const WORLD_TRANSFORM: Transform = Transform::from_xyz(0.0, 0.0, 0.0);
pub const AMBIENT_LIGHT: f32 = 10000.0;

// RAY CASTING
pub const RAY_MAX_DIST: f32 = 10.0;

// TEXTURES
pub const VOXEL_DEFINITITION_PATH: &str = "assets/voxels/voxel_definitions.json";
pub const TEXTURE_PATH: &str = "textures/texturepack2.png";
pub const NUM_VOXELS: usize = 31;
pub const ROTATION_LOCKED_SUBSETS: usize = 2;

pub const SUBSET_SIZES: [usize; 9] = [8, 16, 1, 1, 1, 1, 1, 1, 1];


// UI 
pub const HOTBAR_BORDER_COLOR: Srgba = Srgba::GREEN;