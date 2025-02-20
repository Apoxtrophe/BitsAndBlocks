use bevy::prelude::*;

use bevy::color::palettes::css::*;


// WORLD
pub const WORLD_WIDTH: f32 = 100.0;
pub const WORLD_TRANSFORM: Transform = Transform::from_xyz(0.0, 0.0, 0.0);
pub const AMBIENT_LIGHT: f32 = 10000.0;

// RAY CASTING
pub const RAY_MAX_DIST: f32 = 10.0;
pub const RAY_SPHERE_RADIUS: f32 = 0.5;
pub const RAY_DEBUG: bool = false; 

// TEXTURES
pub const TEXTURE_PATH: &str = "textures/TexturePack6.png";
pub const NUM_TEXTURES: usize = 9;
pub const ROTATION_LOCKED_SUBSETS: usize = 2;
pub const TEXTURE_MAP: [(usize,usize); 9]  = 
    [
        (0,0),      
        (1,0), (1,1), (1,2), (1,3), (1,4), (1,5), (1,6), (1,7)
    ];
pub const SUBSET_SIZES: [usize; 2] = [1, 8];

// UI 
pub const HOTBAR_BORDER_COLOR: Srgba = Srgba::GREEN;