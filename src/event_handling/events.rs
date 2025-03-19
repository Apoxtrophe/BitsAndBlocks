use std::{cmp::Ordering, fmt::Pointer};

use bevy::{input::mouse::MouseWheel, prelude::*, window::CursorGrabMode};
use bevy_fps_controller::controller::FpsController;

use crate::prelude::*;

#[derive(Event)]
pub enum GameEvent {
    PlaceBlock {
        voxel: Voxel,
        voxel_asset: VoxelAsset,
    },
    RemoveBlock {
        position: IVec3,
    },
    UpdateMesh {
        updates: [IVec3; 6],
    },
    UpdateCursor {
        mode: CursorGrabMode,
        show_cursor: bool,
        enable_input: bool,
    },
    SaveWorld {
        world: SavedWorld,
    }
    
}

pub fn event_handler(
    time: Res<Time>,
    mut event_reader: EventReader<GameEvent>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut commands: Commands,
    mut voxel_map: ResMut<VoxelMap>,
    mut player: ResMut<Player>,
    mut controller_query: Query<&mut FpsController>,
    mut window_query: Query<&mut Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(Entity, &Voxel)>,
    mut fade_timer: ResMut<FadeTimer>,
    save_query: Query<(Entity, &Voxel)>,
) {
    for event in event_reader.read() {
        let event_time = time.elapsed_secs();
        match event {
            GameEvent::PlaceBlock { voxel, voxel_asset } => {
                println!("{:.3}      GAME EVENT: PLACE BLOCK", event_time);
                let mut voxel_assets = voxel_asset.clone();

                if voxel.voxel_id.0 == 1 || voxel.voxel_id.0 == 2 { // Figures out the mesh for cable type voxels 
                    let connections = count_neighbors(*voxel, &voxel_map);
                    let image_row = voxel_map.asset_map[&voxel.voxel_id].texture_row;

                    voxel_assets.mesh_handle =
                        meshes.add(create_cable_mesh(image_row, connections));
                }
                add_voxel(
                    &mut commands,
                    &mut voxel_map,
                    voxel_assets.clone(),
                    voxel.clone(),
                );
            }
            GameEvent::RemoveBlock { position } => {
                println!("{:.3}      GAME EVENT: REMOVE BLOCK", event_time);
                remove_voxel(&mut commands, &mut voxel_map, position.clone());
            }
            GameEvent::UpdateCursor {
                mode,
                show_cursor,
                enable_input,
            } => {
                println!("{:.3}      GAME EVENT: UPDATE CURSOR", event_time);
                if let Ok(mut window) = window_query.get_single_mut() {
                    update_cursor_and_input(
                        &mut window,
                        &mut controller_query,
                        *mode,
                        *show_cursor,
                        *enable_input,
                    );
                }
            }
            GameEvent::UpdateMesh { updates } => {
                println!("{:.3}      GAME EVENT: UPDATE MESH", event_time);
                update_meshes(
                    *updates,
                    &mut voxel_map,
                    &mut commands,
                    &mut meshes,
                    &mut query,
                );
            }
            GameEvent::SaveWorld { world } => {
                println!("{:.3}      GAME EVENT: SAVE WORLD", event_time);
                save_world(&save_query, &world).expect("Couldn't Save");
            }
        }
    }

    // Mouse Scroll Events for scrolling hotbar
    for event in evr_scroll.read() {
        match event.y.partial_cmp(&0.0) {
            Some(Ordering::Greater) => {
                // When subtracting 1, add 8 instead (because (x - 1) mod 9 == (x + 8) mod 9)
                player.hotbar_selector = (player.hotbar_selector + 8) % 9;
                fade_timer.timer.reset();
            }
            Some(Ordering::Less) => {
                // Increment and wrap-around automatically
                player.hotbar_selector = (player.hotbar_selector + 1) % 9;
                fade_timer.timer.reset();
            }
            _ => (),
        }
    }
}
