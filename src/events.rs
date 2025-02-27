use std::cmp::Ordering;

use bevy::{input::mouse::MouseWheel, prelude::*, window::CursorGrabMode};
use bevy_fps_controller::controller::FpsController;

use crate::{graphics::create_cable_mesh, player::{update_cursor_and_input, PlayerData}, texture_row, voxel::{add_voxel, count_neighbors, remove_voxel, update_meshes, Voxel, VoxelAsset}, VoxelMap};

#[derive(Event)]
pub enum GameEvent {
    PlaceBlock {voxel: Voxel, voxel_asset: VoxelAsset},
    RemoveBlock {position: IVec3},
    UpdateMeshCall {updates: [IVec3;6]},
    UpdateCursor {
        mode: CursorGrabMode,
        show_cursor: bool,
        enable_input: bool,
    },
}

pub fn event_handler(
    mut event_reader: EventReader<GameEvent>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut commands: Commands,
    mut voxel_map: ResMut<VoxelMap>,
    mut player: ResMut<PlayerData>,
    mut controller_query: Query<&mut FpsController>,
    mut window_query: Query<&mut Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(Entity, &Voxel)>,
) {
    for event in event_reader.read() {
        match event {
            GameEvent::PlaceBlock { voxel, voxel_asset } => {
                let mut voxel_assets = voxel_asset.clone(); 
                
                if voxel.voxel_id.0 == 1 || voxel.voxel_id.0 == 2 {
                    let connections = count_neighbors(*voxel, &voxel_map);
                    let image_row = texture_row(voxel.voxel_id);
                    
                    voxel_assets.mesh_handle = meshes.add(create_cable_mesh(image_row, connections));
                }
                add_voxel(&mut commands, &mut voxel_map, voxel_assets.clone(), voxel.clone());
            }
            GameEvent::RemoveBlock { position } => {
                remove_voxel(&mut commands, &mut voxel_map, position.clone());
            }
            GameEvent::UpdateCursor { mode, show_cursor, enable_input } => {
                if let Ok(mut window) = window_query.get_single_mut() {
                    update_cursor_and_input(&mut window, &mut controller_query, *mode, *show_cursor, *enable_input,);
                }
            }
            GameEvent::UpdateMeshCall { updates} => {
                update_meshes(*updates, &mut voxel_map, &mut commands, &mut meshes, &mut query);
            }
        }
    }
    
    // Mouse Scroll Events for scrolling hotbar
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