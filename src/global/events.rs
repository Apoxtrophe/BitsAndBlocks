
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
    },
    StateChange {
        new_state: GameState,
    },
    ToggleUI {
        new_ui: GameUI,
    } // Toggles the debug information on / off
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
    mut which_ui: ResMut<GameUI>,
) {
    for event in event_reader.read() {
        let event_time = time.elapsed_secs(); // Keeps track of when events happen
        match event {
            GameEvent::PlaceBlock { voxel, voxel_asset } => {
                println!("{:.3}      GAME EVENT: PLACE BLOCK", event_time);
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
                update_meshes(*updates, &mut voxel_map, &mut commands, &mut meshes, &mut voxel_query);
            }
            GameEvent::SaveWorld { world } => {
                println!("{:.3}      GAME EVENT: SAVE WORLD", event_time);
                save_world(&save_query, &world).expect("Couldn't Save");
                return; // Saving world skips other event handling. This is to prevent world state from changing before saving.
            }
            GameEvent::StateChange { new_state } => {
                println!("{:.3}      GAME EVENT: STATE CHANGE", event_time);
                app_state.set(*new_state);
            }
            GameEvent::ToggleUI { new_ui } => {
                *which_ui = *new_ui;
            }
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
