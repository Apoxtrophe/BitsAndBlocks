use std::time::Duration;

use bevy::{prelude::*, window::CursorGrabMode};
use bevy_fps_controller::controller::FpsController;
use bevy_rapier3d::prelude::Velocity;
use crate::prelude::*;

/// Respawn entities whose vertical position falls below the threshold.
pub fn respawn_system(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in query.iter_mut() {
        // Only respawn if the entity is below -10.0
        if transform.translation.y > -10.0 {
            continue;
        }
        velocity.linvel = Vec3::ZERO;
        transform.translation = SPAWN_POINT;
    }
}

/// Update the window's cursor options and the FPS controller's input state.
pub fn update_cursor_and_input(
    window: &mut Window,
    controller_query: &mut Query<&mut FpsController>,
    grab_mode: CursorGrabMode,
    cursor_visible: bool,
    input_enabled: bool,
) {
    window.cursor_options.grab_mode = grab_mode;
    window.cursor_options.visible = cursor_visible;
    for mut controller in controller_query.iter_mut() {
        controller.enable_input = input_enabled;
    }
}

/// Process input events for updating cursor mode and handling player actions.
pub fn input_event_system(
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: ResMut<Player>,
    voxel_assets: Res<VoxelMap>,
    mut window_query: Query<&mut Window>,
    mut event_writer: EventWriter<GameEvent>,
    mut which_ui: ResMut<WhichUIShown>,
    mut place_timer: Local<Timer>,
    mut remove_timer: Local<Timer>,
    time: Res<Time>,
) { 

    
    // --- Cursor and Input Mode Updates ---
    if window_query.get_single_mut().is_ok() {
        if mouse_input.just_pressed(MouseButton::Left)  && false{
            event_writer.send(GameEvent::UpdateCursor {
                mode: CursorGrabMode::Locked,
                show_cursor: false,
                enable_input: true,
            });
        } else if keyboard_input.just_pressed(KeyCode::Escape) {
            if which_ui.ui != WhichGameUI::ExitMenu {
                event_writer.send(GameEvent::UpdateCursor {
                    mode: CursorGrabMode::None,
                    show_cursor: true,
                    enable_input: false,
                });
                which_ui.ui = WhichGameUI::ExitMenu; // Set the UI to the exit menu.
            } else if which_ui.ui == WhichGameUI::ExitMenu {
                event_writer.send(GameEvent::UpdateCursor {
                    mode: CursorGrabMode::Locked,
                    show_cursor: false,
                    enable_input: true,
                });
                which_ui.ui = WhichGameUI::Default; // Set the UI to the exit menu.
            }
        } else if keyboard_input.pressed(KeyCode::Tab) {

            if which_ui.ui != WhichGameUI::ExitMenu {
                which_ui.ui = WhichGameUI::Inventory; // Set the UI to the inventory screen.
                
                event_writer.send(GameEvent::UpdateCursor {
                    mode: CursorGrabMode::Locked,
                    show_cursor: true,
                    enable_input: false,
                });
            }
        } else if keyboard_input.just_released(KeyCode::Tab) {

            if which_ui.ui != WhichGameUI::ExitMenu {
                which_ui.ui = WhichGameUI::Default; // Set the UI to the inventory screen.
                event_writer.send(GameEvent::UpdateCursor {
                    mode: CursorGrabMode::Locked,
                    show_cursor: false,
                    enable_input: true,
                });
            }
        }
    }
    
    let place_delay = Duration::from_millis(PLAYER_PLACE_DELAY);
    let remove_delay = Duration::from_millis(PLAYER_BREAK_DELAY);
    
    // Disable player interaction if they menus are open
    if keyboard_input.pressed(KeyCode::Tab) || which_ui.ui == WhichGameUI::ExitMenu {
        return;
    }

    // --- Player Action Events ---
    if mouse_input.just_pressed(MouseButton::Left) 
    || (mouse_input.pressed(MouseButton::Left) && place_timer.tick(time.delta()).finished()) {
        let mut selected_voxel = match player.selected_voxel {
            Some(voxel) => voxel,
            None => return,
        };

        // Set the direction of the selected voxel based on the camera direction.
        selected_voxel.direction = cardinalize(player.camera_dir);

        let voxel_asset = voxel_assets.asset_map[&selected_voxel.voxel_id].clone();

        event_writer.send(GameEvent::PlaceBlock {
            voxel: selected_voxel,
            voxel_asset,
        });

        // Send mesh update events for neighboring coordinates.2
        let mesh_updates = get_neighboring_coords(selected_voxel.position);
        event_writer.send(GameEvent::UpdateMesh { updates: mesh_updates });
        
        place_timer.reset();
        place_timer.set_duration(place_delay);
        
    } else if mouse_input.just_pressed(MouseButton::Right)
   || (mouse_input.pressed(MouseButton::Right) && remove_timer.tick(time.delta()).finished()) {
        let hit_voxel = match player.hit_voxel {
            Some(voxel) => voxel,
            None => return,
        };

        event_writer.send(GameEvent::RemoveBlock {
            position: hit_voxel.position,
        });

        let mesh_updates = get_neighboring_coords(hit_voxel.position);
        event_writer.send(GameEvent::UpdateMesh { updates: mesh_updates });
        
        remove_timer.reset();
        remove_timer.set_duration(remove_delay);
        
    } else if mouse_input.just_pressed(MouseButton::Middle) { // Middle click to copy looked at voxel. 
        let hit_voxel = match player.hit_voxel {
            Some(voxel) => voxel,
            None => return,
        };
        let voxel_def = voxel_assets.asset_map[&hit_voxel.voxel_id].definition.clone();
        let (set, sub) = voxel_def.voxel_id;
        player.hotbar_selector = set; 
        player.hotbar_ids[set] = (set, sub);
    }
}
