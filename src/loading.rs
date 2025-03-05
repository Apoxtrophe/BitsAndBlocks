use std::{collections::HashMap, fs};

use bevy::prelude::*;

use crate::{config::{CURSOR_TEXTURE_PATH, VOXEL_TEXTURE_PATH, WORLD_TEXTURE_PATH}, ui::create_definition_timer, voxel::create_voxel_map, GameState};

/// Handles Asset and Resource Loading before entering the main menu / Game. 
pub fn loading(
    mut window: Query<&mut Window>,
    mut app_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    // Configure the main window
    let mut window = window.single_mut();
    window.title = String::from("Bits And Blocks");
    
    println!("State: Loading");
    
    let game_texture_handles = GameTextures {
        ground_texture: asset_server.load(WORLD_TEXTURE_PATH),
        cursor_texture: asset_server.load(CURSOR_TEXTURE_PATH),
        voxel_textures: asset_server.load(VOXEL_TEXTURE_PATH),
        home_screen_texture: asset_server.load("textures/homescreen.png"),
        menu_button_texture: asset_server.load("textures/buttons.png"),
        new_game_screen_texture: asset_server.load("textures/new_game.png"),
        load_game_screen_texture: asset_server.load("textures/load_game.png"),
        options_screen_texture: asset_server.load("textures/options_screen.png"),
    };
    
    // Load Game Textures
    commands.insert_resource(game_texture_handles.clone());
    
    // Create VoxelMap    
    commands.insert_resource(create_voxel_map(meshes, materials, game_texture_handles.voxel_textures)); 
    
    // Create Voxel definition text timer
    commands.insert_resource(create_definition_timer());
    
    
    println!("Assets Loaded, Moving to Main Menu");
    
    app_state.set(GameState::MainMenu);
}

#[derive(Resource, Clone)]
pub struct GameTextures {
    pub ground_texture: Handle<Image>,
    pub cursor_texture: Handle<Image>,
    pub voxel_textures: Handle<Image>,
    pub home_screen_texture: Handle<Image>,
    pub menu_button_texture: Handle<Image>,
    pub new_game_screen_texture: Handle<Image>,
    pub load_game_screen_texture: Handle<Image>,
    pub options_screen_texture: Handle<Image>,
}

#[derive(Resource, Clone)]
pub struct VoxelMap {
    pub entity_map: HashMap<IVec3, Entity>, // Entity ids by location
    pub voxel_map: HashMap<IVec3, Voxel>, // Local voxel values by location
    pub asset_map: HashMap<(usize, usize), VoxelAsset>, // global voxel values by id 
}

#[derive(Component, Debug, Copy, Clone)]
pub struct Voxel {
    pub voxel_id: (usize, usize),
    pub position: IVec3,
    pub direction: usize,
    pub state: bool,
}

#[derive(Clone)]
pub struct VoxelAsset {
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<StandardMaterial>,
    pub definition: VoxelDefinition,
    pub texture_row: usize, 
}


#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct VoxelDefinition{
    pub voxel_id: (usize, usize),
    pub name: String,
}

// Fade timer resource for voxel definition text above the hotbar
#[derive(Resource)]
pub struct FadeTimer {
    pub timer: Timer,
}
