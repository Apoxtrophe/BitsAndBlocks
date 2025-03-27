
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

/// Main player input system.
pub fn player_input_system(
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: ResMut<Player>,
    voxel_assets: Res<VoxelMap>,
    mut window_query: Query<&mut Window>,
    mut event_writer: EventWriter<GameEvent>,
    current_ui: ResMut<GameUI>,
    mut place_timer: Local<Timer>,
    mut remove_timer: Local<Timer>,
    time: Res<Time>,
) {
    if *current_ui == GameUI::Default{ // Make it so that you cannot place/destroy when not in default mode ui
        // Process block interactions.
        handle_block_placement(
            &mouse_input,
            &mut player,
            &voxel_assets,
            &mut event_writer,
            &mut place_timer,
            &time,
        );
        handle_block_removal(
            &mouse_input,
            &player,
            &mut event_writer,
            &mut remove_timer,
            &time,
        );
        handle_hotbar_copy(
            &mouse_input,
            &mut player,
            &voxel_assets,
            &mut event_writer,
        );
    }
    // UI inputs require a valid window (if applicable).
    if window_query.get_single_mut().is_ok() {
        process_ui_input(&keyboard_input, current_ui, &mut event_writer);
    } else if keyboard_input.pressed(KeyCode::Tab) || *current_ui == GameUI::ExitMenu {
        return;
    }


}

/// Handles key events for toggling UI modes.
fn process_ui_input(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    mut current_ui: ResMut<GameUI>,
    event_writer: &mut EventWriter<GameEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        // Toggle Exit Menu.
        if *current_ui != GameUI::ExitMenu {
            event_writer.send(GameEvent::UpdateCursor {
                mode: CursorGrabMode::None,
                show_cursor: true,
                enable_input: false,
            });
            *current_ui = GameUI::ExitMenu;
        } else {
            event_writer.send(GameEvent::UpdateCursor {
                mode: CursorGrabMode::Locked,
                show_cursor: false,
                enable_input: true,
            });
            *current_ui = GameUI::Default;
        }
    } else if keyboard_input.just_pressed(KeyCode::Tab) {
        // Open Inventory, if not in the Exit Menu.
        if *current_ui != GameUI::ExitMenu {
            *current_ui = GameUI::Inventory;
            event_writer.send(GameEvent::UpdateCursor {
                mode: CursorGrabMode::Locked,
                show_cursor: true,
                enable_input: false,
            });
        }
    } else if keyboard_input.just_released(KeyCode::Tab) {
        // Close Inventory.
        if *current_ui != GameUI::ExitMenu {
            *current_ui = GameUI::Default;
            event_writer.send(GameEvent::UpdateCursor {
                mode: CursorGrabMode::Locked,
                show_cursor: false,
                enable_input: true,
            });
        }
    } else if keyboard_input.just_pressed(KeyCode::F3) {
        // Toggle Debug mode.
        let new_ui = if *current_ui == GameUI::Debug {
            GameUI::Default
        } else {
            GameUI::Debug
        };
        event_writer.send(GameEvent::ToggleUI { new_ui });
    }
}

/// Handles block placement using left mouse input and a timer.
fn handle_block_placement(
    mouse_input: &Res<ButtonInput<MouseButton>>,
    player: &mut ResMut<Player>,
    voxel_assets: &Res<VoxelMap>,
    event_writer: &mut EventWriter<GameEvent>,
    place_timer: &mut Timer,
    time: &Res<Time>,
) {
    if mouse_input.just_pressed(MouseButton::Left)
        || (mouse_input.pressed(MouseButton::Left) && place_timer.tick(time.delta()).finished())
    {
        if let Some(mut selected_voxel) = player.selected_voxel {
            // Update voxel direction based on the camera.
            selected_voxel.direction = cardinalize(player.camera_dir);

            // Retrieve the voxel asset.
            let voxel_asset = voxel_assets.asset_map[&selected_voxel.voxel_id].clone();

            // Dispatch the block placement event.
            event_writer.send(GameEvent::PlaceBlock {
                voxel: selected_voxel,
                voxel_asset,
            });

            // Also update neighboring meshes.
            let mesh_updates = get_neighboring_coords(selected_voxel.position);
            event_writer.send(GameEvent::UpdateMesh { updates: mesh_updates });

            place_timer.reset();
            place_timer.set_duration(PLAYER_PLACE_DELAY);
        }
    }
}

/// Handles block removal using right mouse input and a timer.
fn handle_block_removal(
    mouse_input: &Res<ButtonInput<MouseButton>>,
    player: &ResMut<Player>,
    event_writer: &mut EventWriter<GameEvent>,
    remove_timer: &mut Timer,
    time: &Res<Time>,
) {
    if mouse_input.just_pressed(MouseButton::Right)
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
}

/// Handles copying voxel data to the hotbar with middle mouse input.
fn handle_hotbar_copy(
    mouse_input: &Res<ButtonInput<MouseButton>>,
    player: &mut ResMut<Player>,
    voxel_assets: &Res<VoxelMap>,
    _event_writer: &mut EventWriter<GameEvent>,
) {
    if mouse_input.just_pressed(MouseButton::Middle) {
        if let Some(hit_voxel) = player.hit_voxel {
            let voxel_def = voxel_assets.asset_map[&hit_voxel.voxel_id].definition.clone();
            let (set, sub) = voxel_def.voxel_id;
            player.hotbar_selector = set;
            player.hotbar_ids[set] = (set, sub);
        }
    }
}
