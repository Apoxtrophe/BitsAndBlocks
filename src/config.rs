use bevy::prelude::*;

// WORLD
pub const WORLD_HEIGHT: f32 = 1.0;
pub const WORLD_WIDTH: f32 = 10.0;
pub const WORLD_TRANSFORM: Transform = Transform::from_xyz(0.0, 0.5, 0.0);

// RAY CASTING
pub const RAY_MAX_DIST: f32 = 100.0;
pub const RAY_SPHERE_RADIUS: f32 = 0.5;