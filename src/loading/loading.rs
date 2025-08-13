use std::fs;

use bevy::{prelude::*, window::{CursorGrabMode, PrimaryWindow}};
use bevy_fps_controller::controller::FpsController;

use crate::{prelude::*, GameState};

/// Handles Asset and Resource Loading before entering the main menu / game.
pub fn loading(
    mut window_q      : Query<&mut Window, With<PrimaryWindow>>,
    mut app_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut controller_query: Query<&mut FpsController>,
) {
    // --- window + cursor ----------------------------------------------------
    if let Ok(mut win) = window_q.get_single_mut() {
        win.title = "Bits & Blocks".into();
        win.cursor_options.visible = true;
        win.cursor_options.grab_mode = CursorGrabMode::None;
    }
    
    // --- schedule simulation timer resource --------------------------------
    commands.insert_resource(SimulationTimer {
        tick : Timer::from_seconds(1.0 / SPEED_SETTINGS[1] as f32, TimerMode::Repeating),
        rate : 1,
    });

    // === Saved Games Resource ===
    let saved_games = load_saved_names();
    commands.insert_resource(saved_games);

    // Disable input for all FPS controllers.
    for mut controller in controller_query.iter_mut() {
        controller.enable_input = false;
    }

    // === Audio Loading ==
    commands.insert_resource(AudioHandles {
        place: asset_server.load(AUDIO_PLACE),
        destroy: asset_server.load(AUDIO_DESTROY),
        ui_hover: asset_server.load(AUDIO_UI_HOVER),
        ui_click: asset_server.load(AUDIO_UI_CLICK),
    });

    // === Texture Loading ===
    let game_texture_handles = GameTextures {
        ground_texture: asset_server.load(WORLD_TEXTURE_PATH),
        voxel_textures: asset_server.load(VOXEL_TEXTURE_PATH),
        home_screen_texture: asset_server.load("textures/homescreen.png"),
        menu_button_texture: asset_server.load("textures/buttons.png"),
        new_game_screen_texture: asset_server.load("textures/new_game.png"),
        load_game_screen_texture: asset_server.load("textures/load_game.png"),
        options_screen_texture: asset_server.load("textures/options_screen.png"),
        speed_indicator_atlas: asset_server.load(SPEED_INDICATOR_PATH),
    };
    commands.insert_resource(game_texture_handles.clone());

    // === Saved World Resource ===
    let saved_world = SavedWorld {
        world_name: String::new(),
        voxels: Vec::new(),
    };
    commands.insert_resource(saved_world);

    // === Voxel Map Creation ===
    commands.insert_resource(create_voxel_map(
        meshes,
        materials,
        game_texture_handles.voxel_textures,
    ));
    
    // === UI Setup ===
    commands.insert_resource(GameUI::MainScreen);
    
    println!("Assets Loaded, Moving to Main Menu");

    // Transition to the main menu state.
    app_state.set(GameState::MainMenu);
}

/// Loads the names of saved games from the assets/saves folder.
/// Scans for files with a `.bin` extension.
fn load_saved_names() -> LoadedSaves {
    let path = "assets/saves/";
    let mut file_names = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("bin") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    file_names.push(stem.to_string());
                }
            }
        }
    }
    LoadedSaves { saves: file_names }
}
