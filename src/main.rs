mod player;
use player::*;

mod voxel;
use voxel::*;

mod ui;
use ui::*;

mod world;
use world::*;

mod raycast;
use raycast::*;

mod config;
use config::*;

//mod ui;
//use ui::*;

use std::f32::consts::TAU;

use bevy::{color::palettes::css::*, 
    prelude::*};
use bevy_rapier3d::prelude::*;

use bevy_fps_controller::controller::*;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 10000.0,
        })
        .insert_resource(PlayerData::default())
        .insert_resource(VoxelMap::default())
        .insert_resource(ClearColor(Color::linear_rgb(0.83, 0.96, 0.96)))
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FpsControllerPlugin)
        .add_systems(Startup, (setup_player, setup)
        )
        .add_systems(
            Update,
            (manage_cursor, respawn, raycast, debug_text, voxel_interaction),
        )
        .run();
}