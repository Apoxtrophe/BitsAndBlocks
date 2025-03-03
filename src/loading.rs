use std::{collections::HashMap, fs};

use bevy::prelude::*;

use crate::{config::{CURSOR_TEXTURE_PATH, VOXEL_DEFINITITION_PATH, VOXEL_TEXTURE_PATH, WORLD_TEXTURE_PATH}, graphics::create_voxel_mesh, voxel::create_voxel_material, GameState};

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
    
    commands.insert_resource(game_texture_handles.clone());
    println!("Loaded Game Textures");
    
    commands.insert_resource(create_voxel_map(meshes, materials, game_texture_handles.voxel_textures)); // Create VoxelMap    

    

    
    app_state.set(GameState::InGame);
}

fn create_voxel_map(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    image_handle: Handle<Image>,
) -> VoxelMap{
    //let texture_handle = asset_server.load(VOXEL_TEXTURE_PATH);
    let file_content = fs::read_to_string(VOXEL_DEFINITITION_PATH)
        .expect("Failed to read file");
    let voxel_defs: Vec<VoxelDefinition> = serde_json::from_str(&file_content)
        .expect("Failed to parse JSON");

    let voxel_asset_map = voxel_defs
        .into_iter()
        .enumerate()
        .map(|(i, voxel_def)| {
            let mesh_handle = meshes.add(create_voxel_mesh(i));
            let material_handle = materials.add(create_voxel_material(image_handle.clone()));
            let texture_row = i;
            (voxel_def.voxel_id, VoxelAsset {
                mesh_handle,
                material_handle,
                definition: voxel_def,
                texture_row,
            })
        })
        .collect::<HashMap<_, _>>();
    
    let entity_map = HashMap::new();
    let voxel_map = HashMap::new();
    
    let voxel_map = VoxelMap {
        entity_map,
        voxel_map,
        asset_map: voxel_asset_map,
    };
    
    voxel_map
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