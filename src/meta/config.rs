use std::time::Duration;

use bevy::prelude::*;

// WORLD
pub const WORLD_TEXTURE_PATH: &str = "textures/ground.png";
pub const WORLD_WIDTH: f32 = 1000.0;
pub const WORLD_TRANSFORM: Transform = Transform::from_xyz(0.0, 0.0, 0.0);
pub const AMBIENT_LIGHT: f32 = 10000.0;

// RAY CASTING
pub const RAY_MAX_DIST: f32 = 10.0;

// TEXTURES
pub const VOXEL_DEFINITITION_PATH: &str = "assets/voxels/voxel_definitions.json";
pub const VOXEL_TEXTURE_PATH: &str = "textures/texturepack2.png";
pub const NUM_VOXELS: usize = 31;
pub const ROTATION_LOCKED_SETS: usize = 2;
pub const SUBSET_SIZES: [usize; 9] = [8, 16, 1, 1, 1, 1, 1, 1, 1];

// UI 
pub const HOTBAR_BORDER_COLOR: Srgba = Srgba::GREEN;
pub const FADE_TIME: f32 = 1.0; // Fade time of Voxel Identifier text
pub const CURSOR_TEXTURE_PATH: &str = "textures/cursor.png";
pub const HOTBAR_SIZE: usize = 9; // Changing this will probably break shit
pub const INVENTORY_SIZE: usize = 16; // 

// PLAYER
pub const PLAYER_HEIGHT: f32 = 1.9;
pub const PLAYER_CROUCHED_HEIGHT: f32 = 1.5;
pub const PLAYER_PLACE_DELAY: Duration = Duration::from_millis(150);
pub const PLAYER_BREAK_DELAY: Duration = Duration::from_millis(150);
pub const SPAWN_POINT: Vec3 = Vec3::new(0.0, 5.625, 0.0);
pub const RESPAWN_THERESHOLD: f32 = -10.0;

// Saving / Loading
pub const TEMP_SAVE_PATH: &str = "assets/saves/temp_save.json";
pub const AUTOSAVE_TIME: Duration = Duration::from_secs(60);
pub const SAVE_SLOTS: usize = 4;