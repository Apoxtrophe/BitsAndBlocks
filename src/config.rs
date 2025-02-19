use bevy::prelude::*;

// WORLD
pub const WORLD_HEIGHT: f32 = 1.0;
pub const WORLD_WIDTH: f32 = 50.0;
pub const WORLD_TRANSFORM: Transform = Transform::from_xyz(0.0, 0.0, 0.0);

// RAY CASTING
pub const RAY_MAX_DIST: f32 = 10.0;
pub const RAY_SPHERE_RADIUS: f32 = 0.5;
pub const RAY_DEBUG: bool = false; 

// TEXTURES
pub const NUM_TEXTURES: usize = 9;
pub const TEXTURE_MAP: [(usize,usize); 9]  = 
    [
        (0,0),      
        (1,0), (1,1), (1,2), (1,3), (1,4), (1,5), (1,6), (1,7)
    ];