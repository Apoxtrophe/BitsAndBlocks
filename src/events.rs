use std::cmp::Ordering;

use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::{player::PlayerData, voxel::{add_voxel, remove_voxel, Voxel, VoxelAsset}, VoxelMap};

#[derive(Event)]
pub enum GameEvent {
    PlaceBlock {voxel: Voxel, voxel_asset: VoxelAsset},
    RemoveBlock {position: IVec3},
}

pub fn event_handler(
    mut event_reader: EventReader<GameEvent>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut commands: Commands,
    mut voxel_map: ResMut<VoxelMap>,
    mut player: ResMut<PlayerData>,
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
    
    // Mouse Scroll Events
    for event in evr_scroll.read() {
        match event.y.partial_cmp(&0.0) {
            Some(Ordering::Greater) => {
                // When subtracting 1, add 8 instead (because (x - 1) mod 9 == (x + 8) mod 9)
                player.selector = (player.selector + 8) % 9;
            },
            Some(Ordering::Less) => {
                // Increment and wrap-around automatically
                player.selector = (player.selector + 1) % 9;
            },
            _ => (),
        }
    }
}