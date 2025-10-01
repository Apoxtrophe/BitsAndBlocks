use std::time::Duration;

use bevy::prelude::*;

// WORLD
pub const WORLD_TEXTURE_PATH: &str = "textures/ground.png";
pub const WORLD_WIDTH: f32 = 1000.0;
pub const WORLD_TRANSFORM: Transform = Transform::from_xyz(0.0, 0.0, 0.0);
pub const AMBIENT_LIGHT: f32 = 10000.0;

// RAY CASTING
pub const MAX_RAY_DIST: f32 = 10.0;

// TEXTURES
pub const VOXEL_DEFINITITION_PATH: &str = "assets/voxels/voxel_definitions.json";
pub const VOXEL_TEXTURE_PATH: &str = "textures/texturesMay5.png";
pub const NUM_VOXELS: usize = 43;
pub const ROTATION_LOCKED_SETS: usize = 2;
pub const SUBSET_SIZES: [usize; 9] = [8, 1, 16, 2, 2, 2, 2, 2, 4];

// UI 
pub const HOTBAR_BORDER_COLOR: Srgba = Srgba::GREEN;
pub const FADE_TIME: f32 = 1.0; // Fade time of Voxel Identifier text
pub const CURSOR_TEXTURE_PATH: &str = "textures/cursor7.png";
pub const HOTBAR_SIZE: usize = 9; // Changing this will probably break shit
pub const INVENTORY_SIZE: usize = 16; // 
pub const SPEED_INDICATOR_PATH: &str = "textures/speed_indicator.png";

pub const PRESSED_COLOR: Color = Color::srgb(0.15, 0.90, 0.15);
pub const HOVER_COLOR: Color = Color::srgb(0.5, 0.60, 0.5);
pub const DEFAULT_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const BORDER_SELECTED: Color = Color::WHITE;
pub const BORDER_UNSELECTED: Color = Color::BLACK;

// PLAYER
pub const PLAYER_HEIGHT: f32 = 1.9;
pub const PLAYER_CROUCHED_HEIGHT: f32 = 1.5;
pub const PLAYER_PLACE_DELAY: Duration = Duration::from_millis(150);
pub const PLAYER_REMOVE_DELAY: Duration = Duration::from_millis(150);
pub const SPAWN_POINT: Vec3 = Vec3::new(0.0, 5.625, 0.0);
pub const RESPAWN_THERESHOLD: f32 = -10.0;

// Saving / Loading
pub const TEMP_SAVE_PATH: &str = "assets/saves/temp_save.json";
pub const AUTOSAVE_TIME: Duration = Duration::from_secs(10);
pub const SAVE_SLOTS: usize = 4;

// AUDIO
pub const AUDIO_PLACE: &str = "audio/place.wav"; 
pub const AUDIO_DESTROY: &str = "audio/destroy.wav";
pub const AUDIO_UI_HOVER: &str = "audio/ui_hover.wav";
pub const AUDIO_UI_CLICK: &str = "audio/ui_click.wav";
pub const MIN_AUDIO_VOLUME: f32 = 0.25; 
pub const MAX_AUDIO_VOLUME: f32 = 1.0; 
pub const UI_VOLUME: f64 = 0.5; 

// SIMULATION
pub const TICK_RATE: u64 = 200;
pub const SPEED_SETTINGS: [u64; 5] = [0, 4, 16, 64, 256]; 