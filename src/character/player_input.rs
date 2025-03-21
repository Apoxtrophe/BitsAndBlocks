
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_fps_controller::controller::FpsController;
use bevy_rapier3d::prelude::Velocity;
use crate::prelude::*;

/// Respawn entities whose vertical position falls below the threshold.
pub fn respawn_system(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in query.iter_mut() {
        // Only respawn if the entity is below RESPAWN_THERESHOLD
        if transform.translation.y > RESPAWN_THERESHOLD {
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
    mut current_ui: ResMut<WhichUIShown>,
    mut place_timer: Local<Timer>,
    mut remove_timer: Local<Timer>,
    time: Res<Time>,
) {
    // --- Cursor and UI Mode Updates ---
    if let Ok(mut _window) = window_query.get_single_mut() {
        // Toggle Exit Menu on Escape key press.
        if keyboard_input.just_pressed(KeyCode::Escape) {
            if current_ui.ui != WhichGameUI::ExitMenu {
                event_writer.send(GameEvent::UpdateCursor {
                    mode: CursorGrabMode::None,
                    show_cursor: true,
                    enable_input: false,
                });
                current_ui.ui = WhichGameUI::ExitMenu;
            } else {
                event_writer.send(GameEvent::UpdateCursor {
                    mode: CursorGrabMode::Locked,
                    show_cursor: false,
                    enable_input: true,
                });
                current_ui.ui = WhichGameUI::Default;
            }
        }
        // Open Inventory on Tab press, if not in the Exit Menu.
        else if keyboard_input.pressed(KeyCode::Tab) {
            if current_ui.ui != WhichGameUI::ExitMenu {
                current_ui.ui = WhichGameUI::Inventory;
                event_writer.send(GameEvent::UpdateCursor {
                    mode: CursorGrabMode::Locked,
                    show_cursor: true,
                    enable_input: false,
                });
            }
        }
        else if keyboard_input.just_pressed(KeyCode::F3) {
        event_writer.send(GameEvent::ToggleDebug{});    
        }
        // Close Inventory on Tab release.
        else if keyboard_input.just_released(KeyCode::Tab) {
            if current_ui.ui != WhichGameUI::ExitMenu {
                current_ui.ui = WhichGameUI::Default;
                event_writer.send(GameEvent::UpdateCursor {
                    mode: CursorGrabMode::Locked,
                    show_cursor: false,
                    enable_input: true,
                });
            }
        }
    }

    // Disable block interactions if certain menus are active.
    if keyboard_input.pressed(KeyCode::Tab) || current_ui.ui == WhichGameUI::ExitMenu {
        return;
    }

    // --- Player Action Events ---
    // Handle block placement (Left Mouse Button).
    if mouse_input.just_pressed(MouseButton::Left)
        || (mouse_input.pressed(MouseButton::Left) && place_timer.tick(time.delta()).finished())
    {
        if let Some(mut selected_voxel) = player.selected_voxel {
            // Update the voxel's direction based on the camera.
            selected_voxel.direction = cardinalize(player.camera_dir);

            // Retrieve the corresponding voxel asset.
            let voxel_asset = voxel_assets.asset_map[&selected_voxel.voxel_id].clone();

            event_writer.send(GameEvent::PlaceBlock {
                voxel: selected_voxel,
                voxel_asset,
            });

            // Send mesh update events for neighboring coordinates.
            let mesh_updates = get_neighboring_coords(selected_voxel.position);
            event_writer.send(GameEvent::UpdateMesh { updates: mesh_updates });

            place_timer.reset();
            place_timer.set_duration(PLAYER_PLACE_DELAY);
        }
    }
    // Handle block removal (Right Mouse Button).
    else if mouse_input.just_pressed(MouseButton::Right)
        || (mouse_input.pressed(MouseButton::Right) && remove_timer.tick(time.delta()).finished())
    {
        if let Some(hit_voxel) = player.hit_voxel {
            event_writer.send(GameEvent::RemoveBlock {
                position: hit_voxel.position,
            });

            let mesh_updates = get_neighboring_coords(hit_voxel.position);
            event_writer.send(GameEvent::UpdateMesh { updates: mesh_updates });

            remove_timer.reset();
            remove_timer.set_duration(PLAYER_BREAK_DELAY);
        }
    }
    // Handle copying voxel data to the hotbar (Middle Mouse Button).
    else if mouse_input.just_pressed(MouseButton::Middle) {
        if let Some(hit_voxel) = player.hit_voxel {
            let voxel_def = voxel_assets.asset_map[&hit_voxel.voxel_id].definition.clone();
            let (set, sub) = voxel_def.voxel_id;
            player.hotbar_selector = set;
            player.hotbar_ids[set] = (set, sub);
        }
    }
}