use std::fmt;
use bevy::{input::mouse::MouseWheel, prelude::*, utils::info, window::CursorGrabMode};
use bevy_fps_controller::controller::FpsController;

use crate::prelude::*;

#[derive(Event, Debug)]
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
    },
    StateChange {
        new_state: GameState,
    },
    ToggleUI {
        new_ui: GameUI,
    },
    LoadWorld {
        world_name: String,
    },
    ChangeGameState {
        new_state: GameState,
    }
}

/// Returns true if the voxel is one of the cable types.
/// Consider replacing magic numbers with named constants or an enum variant.
fn is_cable_voxel(voxel: &Voxel) -> bool {
    voxel.voxel_id.0 == 1 || voxel.voxel_id.0 == 2
}

pub fn event_handler(
    time: Res<Time>,
    mut event_reader: EventReader<GameEvent>,
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    mut commands: Commands,
    mut voxel_map: ResMut<VoxelMap>,
    mut player: ResMut<Player>,
    mut controller_query: Query<&mut FpsController>,
    mut window_query: Query<&mut Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut voxel_query : Query<(Entity, &Voxel)>,
    mut fade_timer: ResMut<FadeTimer>,
    save_query: Query<(Entity, &Voxel)>,
    mut app_state: ResMut<NextState<GameState>>,
    mut this_ui: ResMut<GameUI>,
    mut game_save: ResMut<SavedWorld>,
) {
    for event in event_reader.read() {
        let event_time = time.elapsed_secs(); // Keeps track of when events happen
        println!("{:.4}        Event: {}", event_time, event);
        match event {
            GameEvent::PlaceBlock { voxel, voxel_asset } => {
                let mut voxel_asset_data = voxel_asset.clone();
                if is_cable_voxel(voxel) {
                    // Determine cable connections from neighboring voxels
                    let connections = count_neighbors(*voxel, &voxel_map);
                    let texture_row = voxel_map
                        .asset_map
                        .get(&voxel.voxel_id)
                        .map(|asset| asset.texture_row)
                        .unwrap_or_default();

                    voxel_asset_data.mesh_handle =
                        meshes.add(create_cable_mesh(texture_row, connections));
                }
                add_voxel(&mut commands, &mut voxel_map, voxel_asset_data, voxel.clone());
            }
            GameEvent::RemoveBlock { position } => {
               
                remove_voxel(&mut commands, &mut voxel_map, position.clone());
            }
            GameEvent::UpdateCursor {
                mode,
                show_cursor,
                enable_input,
            } => {
                
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
                
                update_meshes(*updates, &mut voxel_map, &mut commands, &mut meshes, &mut voxel_query);
            }
            GameEvent::SaveWorld { world } => {
                
                save_world(&save_query, &world).expect("Couldn't Save");
                return; // Saving world skips other event handling. This is to prevent world state from changing before saving.
            }
            GameEvent::LoadWorld { world_name } => {
                load_world(world_name, &mut commands, &mut voxel_map, &mut meshes);
                return; // Loading world skips other event handling. This is to prevent world state from changing before loading.
            }
            GameEvent::StateChange { new_state } => {
                app_state.set(*new_state);
            }
            GameEvent::ToggleUI { new_ui } => {
                *this_ui = *new_ui;
            }
            _ => {println!("!!!UN-HANDLED EVENT")}
        }
    }

    // Mouse Scroll Events for scrolling hotbar
    for event in mouse_wheel_reader.read() {
        if event.y > 0.0 {
            // Decrement hotbar selector with wrap-around
            player.hotbar_selector = (player.hotbar_selector + (HOTBAR_SIZE - 1)) % HOTBAR_SIZE;
            fade_timer.timer.reset();
        } else if event.y < 0.0 {
            // Increment hotbar selector with wrap-around
            player.hotbar_selector = (player.hotbar_selector + 1) % HOTBAR_SIZE;
            fade_timer.timer.reset();
        }
    }
}

impl fmt::Display for GameEvent { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameEvent::PlaceBlock { voxel, voxel_asset } => {
                write!(f, "EVENT VOXEL PLACE: {:?}  {:?}", voxel.voxel_id, voxel.position)
            }
            GameEvent::RemoveBlock { position } => {
                write!(f, "EVENT VOXEL REMOVE: {:?}", position)
            }
            GameEvent::UpdateMesh { updates } => {
                write!(f, "EVENT MESH UPDATE: {:?}", updates)
            }
            GameEvent::UpdateCursor { mode, show_cursor, enable_input } => {
                write!(f, "EVENT CURSOR: {:?}, show_cursor: {}, enable_input: {}", mode, show_cursor, enable_input)
            }
            GameEvent::SaveWorld { world } => {
                write!(f, "EVENT SAVE WORLD: {:?}", world)
            }
            GameEvent::StateChange { new_state } => {
                write!(f, "EVENT STATE CHANGE: {:?}", new_state)
            }
            GameEvent::ToggleUI { new_ui } => {
                write!(f, "EVENT TOGGLE UI: {:?}", new_ui)
            }
            GameEvent::LoadWorld { world_name } => {
                write!(f, "EVENT LOAD WORLD: {:?}", world_name)
            }
            GameEvent::ChangeGameState { new_state } => {
                write!(f, "EVENT CHANGE GAME STATE: {:?}", new_state)
            }
        }
    }
}