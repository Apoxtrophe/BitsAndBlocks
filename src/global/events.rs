use std::{fmt, time::Duration};
use bevy::{input::mouse::MouseWheel, prelude::*, window::CursorGrabMode};
use bevy_fps_controller::controller::FpsController;

use crate::prelude::*;

#[derive(Event, Debug)]
pub enum GameEvent {
    Skip{}, // Event flag that skips the current iteration of the event handler
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
    UpdateCursorMode {
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
    DeleteWorld {
        world_name: String,
    },
    ModifyPlayer {
        player_modified: Player,
    },
    SpeedChange {
      change: i32,   
    }
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
    mut voxel_query: Query<(Entity, &Voxel)>,
    save_query: Query<(Entity, &Voxel)>,
    mut app_state: ResMut<NextState<GameState>>,
    mut this_ui: ResMut<GameUI>,
    mut game_save: ResMut<SavedWorld>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut simulation_time: ResMut<SimulationTimer>,
) {
    // Process game events.
    for event in event_reader.read() {
        let event_time = time.elapsed_secs();
        println!("{:.4}         {:?}", event_time, event);
        match event {
            GameEvent::Skip { .. } => {
                return;
            }
            GameEvent::PlaceBlock { voxel, voxel_asset } => {
                let mut voxel_asset_data = voxel_asset.clone();
                
                let is_valid = matches!(voxel.kind, VoxelType::Wire(_) | VoxelType::BundledWire);
                if is_valid {
                    // Determine cable connections from neighboring voxels.
                    let connections = count_neighbors(*voxel, &voxel_map);
                    let texture_row = voxel_map
                        .asset_map
                        .get(&voxel.kind)
                        .map(|asset| asset.texture_row)
                        .unwrap_or_default();

                    voxel_asset_data.mesh_handle =
                        meshes.add(create_cable_mesh(texture_row, connections));
                }
                let new_voxel = voxel.clone();
                add_voxel(&mut commands, &mut voxel_map, voxel_asset_data, new_voxel, &mut materials);
            }
            GameEvent::RemoveBlock { position } => {
                remove_voxel(&mut commands, &mut voxel_map, position.clone());
            }
            GameEvent::UpdateCursorMode {
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
                game_save.world_name = world.world_name.clone();
                save_world(&save_query, &world).expect("Couldn't Save");
            }
            GameEvent::LoadWorld { world_name } => {
                game_save.world_name = world_name.clone();
                load_world(world_name, &mut commands, &mut voxel_map, &mut meshes, &mut materials);
            }
            GameEvent::DeleteWorld { world_name } => {
                delete_world(world_name);
            }
            GameEvent::StateChange { new_state } => {
                app_state.set(*new_state);
            }
            GameEvent::ToggleUI { new_ui } => {
                *this_ui = *new_ui;
            }
            GameEvent::ModifyPlayer { player_modified: modified } => {
                *player = modified.clone();
            }
            GameEvent::SpeedChange { change } => {
                let current = simulation_time.rate;
                let new_rate = (current as i32 + change).clamp(0, SPEED_SETTINGS.len() as i32 - 1) as usize;
                let sim_rate  = if new_rate == 0 { 100000.0 } else { 1.0 / SPEED_SETTINGS[new_rate] as f32};
                simulation_time.rate = new_rate as u64;
                simulation_time.tick.set_duration(Duration::from_secs_f32(sim_rate));
            }
            _ => {
                println!("!!!UN-HANDLED EVENT");
            }
        }
    }
    // Handle mouse scroll events for scrolling the hotbar.
    for event in mouse_wheel_reader.read() {
        let step = event.y.signum() as isize;
        player.hotbar_selector = ((player.hotbar_selector as isize) - step)
            .rem_euclid(HOTBAR_SIZE as isize) as usize;
    }
}

impl fmt::Display for GameEvent { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameEvent::Skip {  } => {
                write!(f, "EVENT SKIP")
            }
            GameEvent::PlaceBlock { voxel, voxel_asset } => {
                write!(f, "EVENT VOXEL PLACE: {:?}  {:?}", voxel.kind, voxel.position)
            }
            GameEvent::RemoveBlock { position } => {
                write!(f, "EVENT VOXEL REMOVE: {:?}", position)
            }
            GameEvent::UpdateMesh { updates } => {
                write!(f, "EVENT MESH UPDATE: {:?}", updates)
            }
            GameEvent::UpdateCursorMode { mode, show_cursor, enable_input } => {
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
            GameEvent::DeleteWorld { world_name } => {
                write!(f, "EVENT DELETE WORLD: {:?}", world_name)
            }
            GameEvent::ModifyPlayer { player_modified } => {
                write!(f, "EVENT MODIFY PLAYER")
            }
            GameEvent::SpeedChange { change } => {
                write!(f, "EVENT SPEED CHANGE: {:?}", change)
            }
        }
    }
}