use std::{
    error::Error,
    fs::File,
    io::{BufReader, Write},
};
use bincode;
use bevy::prelude::*;

use crate::prelude::*;

pub fn save_world(
    query: &Query<(Entity, &Voxel)>,
    save_game: &SavedWorld,
) -> Result<(), Box<dyn Error>> {
    // Collect all voxels from the current world.
    let voxels: Vec<Voxel> = query.iter().map(|(_, voxel)| voxel.clone()).collect();

    let saved_world = SavedWorld {
        world_name: save_game.world_name.clone(),
        voxels,
    };

    // Serialize the saved world using bincode.
    let serialized = bincode::serde::encode_to_vec(&saved_world, bincode::config::standard())?;
    
    // Save the serialized data to a file.
    let file_path = format!("assets/saves/{}.bin", save_game.world_name);
    let mut file = File::create(file_path)?;
    file.write_all(&serialized)?;

    Ok(())
}

pub fn autosave_system(
    mut autosave_timer: Local<Timer>,
    time: Res<Time>,
    save_game: Res<SavedWorld>,
    mut event_writer: EventWriter<GameEvent>,
) {
    autosave_timer.tick(time.delta());
    if autosave_timer.finished() {
        event_writer.send(GameEvent::SaveWorld {
            world: save_game.clone(),
        });
        autosave_timer.set_duration(AUTOSAVE_TIME);
        autosave_timer.reset();
    }
}

pub fn load_world(
    world_name: &str,
    commands: &mut Commands,
    mut voxel_map: &mut ResMut<VoxelMap>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let file_path = format!("assets/saves/{}.bin", world_name);
    let file = File::open(&file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    // Deserialize the saved world.
    let saved_world: SavedWorld = bincode::serde::decode_from_reader(reader, bincode::config::standard())
        .expect("Couldn't decode saved world");

    // Add each voxel from the saved world to the voxel map.
    for voxel in &saved_world.voxels {
        let voxel_id = voxel.voxel_id;
        let voxel_asset = voxel_map
            .asset_map
            .get(&voxel_id)
            .expect("Failed to get voxel asset")
            .clone();
        add_voxel(commands, &mut voxel_map, voxel_asset, voxel.clone(), materials);
    }

    // Update cable meshes for voxels identified as cables.
    for voxel in &saved_world.voxels {
        if is_cable_voxel(voxel) {
            let voxel_pos = voxel.position;
            let entity = voxel_map
                .entity_map
                .get(&voxel_pos)
                .expect("Failed to get entity")
                .clone();
            update_voxel_cable_mesh(entity, voxel, &voxel_map, &mut meshes, commands);
        }
    }
}


pub fn delete_world(world_name: &str) {
    let file_path = format!("assets/saves/{}.bin", world_name);
    std::fs::remove_file(file_path).expect("Failed to delete file");
}
