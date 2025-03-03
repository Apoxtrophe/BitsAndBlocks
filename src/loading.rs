use bevy::prelude::*;

use crate::{config::{CURSOR_TEXTURE_PATH, VOXEL_TEXTURE_PATH, WORLD_TEXTURE_PATH}, GameState};

#[derive(Resource)]
pub struct GameTextures {
    pub ground_texture: Handle<Image>,
    pub cursor_texture: Handle<Image>,
    pub voxel_textures: Handle<Image>,
}

pub fn setup_main_menu(
    mut window: Query<&mut Window>,
    mut app_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Configure the main window
    let mut window = window.single_mut();
    window.title = String::from("Bits And Blocks");
    
    let game_texture_handles = GameTextures {
        ground_texture: asset_server.load(WORLD_TEXTURE_PATH),
        cursor_texture: asset_server.load(CURSOR_TEXTURE_PATH),
        voxel_textures: asset_server.load(VOXEL_TEXTURE_PATH),
    };
    
    commands.insert_resource(game_texture_handles);
    
    println!("Loaded Game Textures");
    
    app_state.set(GameState::InGame);
}