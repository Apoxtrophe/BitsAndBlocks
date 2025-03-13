use std::{error::Error, fs::File, io::{BufReader, Write}};
use serde_json;

use bevy::prelude::*;

use crate::{config::AUTOSAVE_TIME, loading::{Voxel, VoxelMap}, voxel::{add_voxel, update_voxel_cable_mesh}};

#[derive(serde::Serialize, serde::Deserialize, Debug, Resource, Clone)]
pub struct SavedWorld {
    pub world_name: String,
    pub voxels: Vec<Voxel>,
}

fn save_world(
    query: Query<(Entity, &Voxel)>,
    save_game: Res<SavedWorld>,
) -> Result<(), Box<dyn Error>> {  // <-- Change return type to Result
    let world_data: Vec<_> = query.iter().map(|(_, voxel)| voxel.clone()).collect();

    let saved_world = SavedWorld {
        world_name: save_game.world_name.clone(),
        voxels: world_data,
    };
    
    let serialized = serde_json::to_string(&saved_world)?; // <-- Uses ? operator
    let file_path = format!("assets/saves/{}.json", save_game.world_name);
    
    let mut file = File::create(file_path)?; // <-- Uses ? operator
    file.write_all(serialized.as_bytes())?; // <-- Uses ? operator
    
    Ok(()) // <-- Explicitly return Ok(())
}

pub fn autosave_system(
    query: Query<(Entity, &Voxel)>,
    mut autosave_timer: Local<Timer>,
    time: Res<Time>,
    save_game: Res<SavedWorld>,
) {
    autosave_timer.set_duration(AUTOSAVE_TIME);
    autosave_timer.tick(time.delta());
    
    if autosave_timer.finished(){
        println!("Saving World...");
        save_world(query, save_game);
        
        autosave_timer.reset();
    }
}
 
pub fn load_world(
    world_name: &str,
    mut commands: Commands,
    mut voxel_map: ResMut<VoxelMap>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let file_path = format!("assets/saves/{}.json", world_name);
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let voxels: SavedWorld = serde_json::from_reader(reader).expect("Failed to read file");
    
    let voxel_list = voxels.voxels;
    
    for i in 0..voxel_list.len() {
        let voxel = voxel_list[i];
        let voxel_id = voxel.voxel_id;
        let voxel_asset = voxel_map.asset_map.get(&voxel_id).expect("Failed to get voxel asset").clone();
        add_voxel(&mut commands, &mut voxel_map, voxel_asset, voxel);
    }
    
    for i in 0..voxel_list.len() {
        let voxel = voxel_list[i];
        let voxel_id = voxel.voxel_id;
        let voxel_pos = voxel.position;
        if voxel_id.0 == 1 || voxel_id.0 == 2 {
            let entity = voxel_map.entity_map.get(&voxel_pos).expect("Failed to get entity").clone();
            update_voxel_cable_mesh(entity, &voxel, &voxel_map, &mut meshes, &mut commands);
        }
    }
}