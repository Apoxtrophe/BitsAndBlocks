use bevy::prelude::*;

// WORLD
pub const WORLD_WIDTH: f32 = 100.0;
pub const WORLD_TRANSFORM: Transform = Transform::from_xyz(0.0, 0.0, 0.0);
pub const AMBIENT_LIGHT: f32 = 10000.0;

// RAY CASTING
pub const RAY_MAX_DIST: f32 = 10.0;
pub const RAY_SPHERE_RADIUS: f32 = 0.25;

// TEXTURES
pub const TEXTURE_PATH: &str = "textures/TexturePack8.png";
pub const NUM_VOXELS: usize = 16;
pub const ROTATION_LOCKED_SUBSETS: usize = 0;
pub const TEXTURE_MAP: [(usize,usize); 16]  = 
    [     
        (0,0), (0,1), (0,2), (0,3), (0,4), (0,5), (0,6), (0,7),
        (1,0),
        (2,0),
        (3,0),
        (4,0),
        (5,0),
        (6,0),
        (7,0),
        (8,0),
    ];
pub const SUBSET_SIZES: [usize; 9] = [8, 1, 1, 1, 1, 1, 1, 1, 1];

// UI 
pub const HOTBAR_BORDER_COLOR: Srgba = Srgba::GREEN;