use std::fs;

use bevy::{prelude::*, window::CursorGrabMode};
use bevy_fps_controller::controller::FpsController;

use crate::{prelude::*, GameState};

/// Handles Asset and Resource Loading before entering the main menu / Game.
pub fn loading(
    mut window: Query<&mut Window>,
    mut app_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut controller_query: Query<&mut FpsController>,
) {
    // Configure the main window
    let mut window = window.single_mut();
    window.title = String::from("Bits And Blocks");

    window.cursor_options = bevy::window::CursorOptions {
        visible: true,
        grab_mode: CursorGrabMode::None,

        ..Default::default()
    };

    let saved_games = load_saved_names();
    commands.insert_resource(saved_games);

    for mut controller in controller_query.iter_mut() {
        controller.enable_input = false;
    }

    println!("State: Loading");

    let game_texture_handles = GameTextures {
        ground_texture: asset_server.load(WORLD_TEXTURE_PATH),
        cursor_texture: asset_server.load(CURSOR_TEXTURE_PATH),
        voxel_textures: asset_server.load(VOXEL_TEXTURE_PATH),
        home_screen_texture: asset_server.load("textures/homescreen.png"),
        menu_button_texture: asset_server.load("textures/buttons.png"),
        new_game_screen_texture: asset_server.load("textures/new_game.png"),
        load_game_screen_texture: asset_server.load("textures/load_game.png"),
        options_screen_texture: asset_server.load("textures/options_screen.png"),
    };

    let saved_world = SavedWorld {
        world_name: "".to_string(),
        voxels: Vec::new(),
    };
    commands.insert_resource(saved_world);

    // Load Game Textures
    commands.insert_resource(game_texture_handles.clone());

    // Create VoxelMap
    commands.insert_resource(create_voxel_map(
        meshes,
        materials,
        game_texture_handles.voxel_textures,
    ));

    // Create Voxel definition text timer
    commands.insert_resource(create_identifier_timer());

    println!("Assets Loaded, Moving to Main Menu");

    app_state.set(GameState::MainMenu);
}

fn load_saved_names() -> LoadedSaves {
    // Define the folder path where your JSON save files are located.
    let path = "assets/saves/";
    let mut file_names = Vec::new();

    // Read the directory and collect JSON file names (without the .json extension).
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            // Check if the file has a ".json" extension.
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                // Extract the file stem (name without extension).
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    file_names.push(stem.to_string());
                }
            }
        }
    }

    // We want exactly 4 slots; if fewer files are found, fill remaining with None.
    const NUM_FILES: usize = 6;
    let mut saves = Vec::with_capacity(NUM_FILES);
    for i in 0..NUM_FILES {
        saves.push(file_names.get(i).cloned());
    }

    LoadedSaves { saves }
}
