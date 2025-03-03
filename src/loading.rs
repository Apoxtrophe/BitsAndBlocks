use std::{collections::HashMap, fs};

use bevy::prelude::*;

use crate::{config::{CURSOR_TEXTURE_PATH, VOXEL_TEXTURE_PATH, WORLD_TEXTURE_PATH}, ui::create_definition_timer, voxel::create_voxel_map, GameState};

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
    
    let game_texture_handles = GameTextures {
        ground_texture: asset_server.load(WORLD_TEXTURE_PATH),
        cursor_texture: asset_server.load(CURSOR_TEXTURE_PATH),
        voxel_textures: asset_server.load(VOXEL_TEXTURE_PATH),
    };
    
    // Load Game Textures
    commands.insert_resource(game_texture_handles.clone());
    
    // Create VoxelMap    
    commands.insert_resource(create_voxel_map(meshes, materials, game_texture_handles.voxel_textures)); 
    
    // Create Voxel definition text timer
    commands.insert_resource(create_definition_timer());
    
    app_state.set(GameState::InGame);
}

#[derive(Resource, Clone)]
pub struct GameTextures {
    pub ground_texture: Handle<Image>,
    pub cursor_texture: Handle<Image>,
    pub voxel_textures: Handle<Image>,
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
