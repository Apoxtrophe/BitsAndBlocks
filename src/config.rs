use bevy::prelude::*;

// WORLD
pub const WORLD_WIDTH: f32 = 100.0;
pub const WORLD_TRANSFORM: Transform = Transform::from_xyz(0.0, 0.0, 0.0);
pub const AMBIENT_LIGHT: f32 = 10000.0;

// RAY CASTING
pub const RAY_MAX_DIST: f32 = 10.0;

// TEXTURES
pub const TEXTURE_PATH: &str = "textures/texturepack2.png";
pub const NUM_VOXELS: usize = 31;
pub const ROTATION_LOCKED_SUBSETS: usize = 2;
pub const VOXEL_LIST: [(usize,usize); 31]  = 
    [     
        (0,0), (0,1), (0,2), (0,3), (0,4), (0,5), (0,6), (0,7),
        (1,0), (1,1), (1,2), (1,3), (1,4), (1,5), (1,6), (1,7), (1,8), (1,9), (1,10), (1,11), (1,12), (1,13), (1,14), (1,15),
        (2,0),
        (3,0),
        (4,0),
        (5,0),
        (6,0),
        (7,0),
        (8,0),
    ];
pub const SUBSET_SIZES: [usize; 9] = [8, 16, 1, 1, 1, 1, 1, 1, 1];


// UI 
pub const HOTBAR_BORDER_COLOR: Srgba = Srgba::GREEN;