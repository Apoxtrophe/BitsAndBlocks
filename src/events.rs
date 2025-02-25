use bevy::prelude::*;

use crate::{voxel::{add_voxel, remove_voxel, Voxel, VoxelAsset}, VoxelMap};

#[derive(Event)]
pub enum GameEvent {
    PlaceBlock {voxel: Voxel, voxel_asset: VoxelAsset},
    RemoveBlock {position: IVec3},
}

pub fn event_handler(
    mut event_reader: EventReader<GameEvent>,
    mut commands: Commands,
    mut voxel_map: ResMut<VoxelMap>,
) {
    for event in event_reader.read() {
        match event {
            GameEvent::PlaceBlock { voxel, voxel_asset } => {
                add_voxel(&mut commands, &mut voxel_map, voxel_asset.clone(), voxel.clone());
            }
            GameEvent::RemoveBlock { position } => {
                remove_voxel(&mut commands, &mut voxel_map, position.clone());
            }
        }
    }
}