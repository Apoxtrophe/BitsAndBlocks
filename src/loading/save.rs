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
    println!("Made it here");
    let world_data: Vec<_> = query.iter().map(|(_, voxel)| voxel.clone()).collect();

    let saved_world = SavedWorld {
        world_name: save_game.world_name.clone(),
        voxels: world_data,
    };

    // Serialize the saved world using bincode for a compact binary format
    //let serialized = bincode::serialize(&saved_world)?;
    let serialized = bincode::serde::encode_to_vec(&saved_world, bincode::config::standard())?;
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
    autosave_timer.set_duration(AUTOSAVE_TIME);
    autosave_timer.tick(time.delta());

    if autosave_timer.finished() {
        event_writer.send(GameEvent::SaveWorld {
            world: save_game.clone(),
        });
        autosave_timer.reset();
    }
}

pub fn load_world(
    world_name: &str,
    mut commands: Commands,
    mut voxel_map: ResMut<VoxelMap>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    println!("Loaded: {}", world_name);
    let file_path = format!("assets/saves/{}.bin", world_name);
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    //let saved_world: SavedWorld = bincode::deserialize_from(reader).expect("Failed to read file");
    let saved_world: SavedWorld = bincode::serde::decode_from_reader(reader, bincode::config::standard()).expect("Couldn't decode");

    // Iterate through voxels and add them to the world
    for voxel in saved_world.voxels.iter() {
        let voxel_id = voxel.voxel_id;
        let voxel_asset = voxel_map
            .asset_map
            .get(&voxel_id)
            .expect("Failed to get voxel asset")
            .clone();
        add_voxel(&mut commands, &mut voxel_map, voxel_asset, voxel.clone());
    }

    // Update the cable meshes for specific voxels
    for voxel in saved_world.voxels.iter() {
        let voxel_id = voxel.voxel_id;
        if voxel_id.0 == 1 || voxel_id.0 == 2 {
            let voxel_pos = voxel.position;
            let entity = voxel_map
                .entity_map
                .get(&voxel_pos)
                .expect("Failed to get entity")
                .clone();
            update_voxel_cable_mesh(entity, voxel, &voxel_map, &mut meshes, &mut commands);
        }
    }
}
