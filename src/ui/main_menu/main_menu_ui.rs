use bevy::window::CursorGrabMode;
use bevy_fps_controller::controller::FpsController;

use crate::prelude::*;

pub fn setup_main_menu(
    mut commands: Commands,
    image_handles: Res<GameTextures>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    saved_games: Res<LoadedSaves>,
) {
    // Spawn the camera tagged for the main menu.
    commands.spawn(Camera2d).insert(MainMenuEntity);

    // Prepare the button texture atlas.
    let buttons_texture = image_handles.menu_button_texture.clone();
    let button_atlas = TextureAtlasLayout::from_grid(UVec2::new(144, 32), 1, 16, None, None);
    let button_atlas_handle = texture_atlases.add(button_atlas);

    // Spawn the main UI window (the root node of the UI).
    let main_ui = spawn_main_ui(&mut commands, &image_handles, &buttons_texture, &button_atlas_handle);

    // Spawn additional UI windows and attach them to the main UI.
    let new_game_window = spawn_new_game_ui(&mut commands, &image_handles, &buttons_texture, &button_atlas_handle);
    commands.entity(new_game_window).set_parent(main_ui);

    let load_game_window = spawn_load_game_ui(&mut commands, &image_handles, &saved_games.saves);
    commands.entity(load_game_window).set_parent(main_ui);

    let options_window = spawn_options_ui(&mut commands, &image_handles);
    commands.entity(options_window).set_parent(main_ui);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuEntity>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    // Update the window's cursor options.
    let mut window = window_query.single_mut();
    window.cursor_options = bevy::window::CursorOptions {
        visible: false,
        grab_mode: CursorGrabMode::Locked,
        ..Default::default()
    };

    // Re-enable input for all FPS controllers.
    for mut controller in controller_query.iter_mut() {
        controller.enable_input = true;
    }

    // Despawn all entities related to the main menu.
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
