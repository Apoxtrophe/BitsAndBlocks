use std::collections::HashMap;

use bevy::prelude::*;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::prelude::*;

#[derive(Resource)]
pub struct Player {
    pub camera_pos: Vec3,
    pub camera_dir: Vec3,
    pub ray_hit_pos: Vec3,
    pub hit_voxel: Option<Voxel>,
    pub selected_voxel: Option<Voxel>,
    pub selected_descriptor: Option<VoxelDefinition>,
    pub hotbar_selector: usize,
    pub hotbar_ids: Vec<(usize, usize)>,
}

impl Default for Player {
    fn default() -> Self {
        let mut hotbar_ids = Vec::new();
        for i in 0..9 {
            hotbar_ids.push((i, 0));
        }
        Self {
            camera_pos: Vec3::ZERO,
            camera_dir: Vec3::ZERO,
            ray_hit_pos: Vec3::ZERO,
            hit_voxel: None,
            selected_voxel: None,
            selected_descriptor: None,
            hotbar_selector: 0,
            hotbar_ids,
        }
    }
}

#[derive(Resource, PartialEq, Eq, Clone, Copy)]
pub struct WhichUIShown {
    pub ui: WhichGameUI,
}

/// Resource for keeping track of which main_menu ui is shown
#[derive(Resource, Debug)]
pub struct WhichScreen {
    pub screen: WhichMenuUI,
}


#[derive(Resource, Clone)]
pub struct VoxelMap {
    pub entity_map: HashMap<IVec3, Entity>, // Entity ids by location
    pub voxel_map: HashMap<IVec3, Voxel>,   // Local voxel values by location
    pub asset_map: HashMap<(usize, usize), VoxelAsset>, // global voxel values by id
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
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
pub struct VoxelDefinition {
    pub voxel_id: (usize, usize),
    pub name: String,
}

#[derive(Resource)]
pub struct LoadedSaves {
    pub saves: Vec<Option<String>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Resource, Clone)]
pub struct SavedWorld {
    pub world_name: String,
    pub voxels: Vec<Voxel>,
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

// Fade timer resource for voxel definition text above the hotbar
#[derive(Resource)]
pub struct FadeTimer {
    pub timer: Timer,
}