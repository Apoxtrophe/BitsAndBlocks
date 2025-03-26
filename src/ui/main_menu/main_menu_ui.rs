use bevy::window::CursorGrabMode;
use bevy_fps_controller::controller::FpsController;
use bevy_simple_text_input::TextInputSubmitEvent;

use crate::{prelude::*, GameState};

pub fn setup_main_menu(
    mut commands: Commands,
    image_handles: Res<GameTextures>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    saved_games: Res<LoadedSaves>,
) {
    println!("State: Main Menu");

    // Spawn the camera tagged for the main menu
    commands.spawn(Camera2d).insert(MainMenuEntity);

    // Prepare button texture atlas
    let buttons_texture = image_handles.menu_button_texture.clone();
    let button_atlas = TextureAtlasLayout::from_grid(UVec2::new(144, 32), 1, 16, None, None);
    let button_atlas_handle = texture_atlases.add(button_atlas);

    // Spawn the main ui Window (The root node of the rest)
    let main_ui = spawn_main_ui(
        &mut commands,
        &image_handles,
        &buttons_texture,
        &button_atlas_handle,
    );

    // Spawn the New Game Window
    let new_game_window = spawn_new_game_ui(
        &mut commands,
        &image_handles,
        &buttons_texture,
        &button_atlas_handle,
    );
    commands.entity(new_game_window).set_parent(main_ui);

    // Spawn the Load Game Window
    let load_game_window =
        spawn_load_game_ui(&mut commands, &image_handles, (0, 6), &saved_games.saves);
    commands.entity(load_game_window).set_parent(main_ui);

    let options_window = spawn_options_ui(&mut commands, &image_handles);
    commands.entity(options_window).set_parent(main_ui);
}

/// System to update button colors based on user interaction.
/// Add this system to your Update schedule.
pub fn menu_interaction_system(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonIdentity),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_ui: ResMut<GameUI>,
    mut exit: EventWriter<AppExit>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<GameState>>,
    save_world: ResMut<SavedWorld>,
    events: EventReader<TextInputSubmitEvent>,
) {
    for (interaction, mut bg_color, button_number) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = Color::linear_rgba(0.0, 1.0, 0.0, 1.0).into();
                match button_number {
                    ButtonIdentity::NewGame => {
                        println!("!!!{:?}", button_number);
                        *game_ui = GameUI::NewGame;
                    }
                    ButtonIdentity::LoadGame => {
                        println!("Load Game");
                        *game_ui = GameUI::LoadGame;
                    }
                    ButtonIdentity::Options => {
                        println!("Options");
                        *game_ui = GameUI::Options;
                    }
                    ButtonIdentity::QuitGame => {
                        println!("Quit Game");
                        exit.send(AppExit::Success);
                    }
                    ButtonIdentity::CreateWorld => {
                        println!("Create World");
                        if save_world.world_name.len() > 0 {
                            *game_ui = GameUI::Default;
                            app_state.set(GameState::InGame);
                        } else {
                            *bg_color = Color::linear_rgba(1.0, 0.0, 0.0, 1.0).into();
                        }
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                *bg_color = Color::linear_rgba(1.0, 1.0, 1.0, 1.0).into();
            }
            Interaction::None => {
                *bg_color = Color::linear_rgba(0.5, 0.5, 0.5, 0.25).into();
            }
        }
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        println!("Main Menu");
        *game_ui = GameUI::MainScreen;
    }

    edit_text_listener(events, save_world);
}

/// Load World button System
pub fn load_world_button_system(
    mut world_button_query: Query<
        (&Interaction, &mut BackgroundColor, &WorldButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<NextState<GameState>>,
    mut game_ui: ResMut<GameUI>,
    commands: Commands,
    voxel_map: ResMut<VoxelMap>,
    meshes: ResMut<Assets<Mesh>>,
    mut game_save: ResMut<SavedWorld>,
) {
    let mut loaded_world: Option<&str> = None;
    for (interaction, mut bg_color, world_button) in world_button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match world_button.name.as_str() {
                    "" => {
                        println!("Empty Slot");
                    }
                    _ => {
                        println!("Load Game: {}", world_button.name);
                        loaded_world = Some(world_button.name.as_str());
                    }
                }
                *bg_color = Color::linear_rgba(0.0, 1.0, 0.0, 0.5).into();
            }
            Interaction::Hovered => {
                *bg_color = Color::linear_rgba(1.0, 1.0, 1.0, 0.5).into();
            }
            Interaction::None => {
                *bg_color = Color::linear_rgba(0.5, 0.5, 0.5, 0.25).into();
            }
        }
    }

    if loaded_world.is_some() {
        load_world(loaded_world.unwrap(), commands, voxel_map, meshes);
        game_save.world_name = loaded_world.unwrap().to_string();
        *game_ui = GameUI::Default;
        app_state.set(GameState::InGame);
        
    }
}

pub fn despawn_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuEntity>>,
    mut window: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    let mut twindow = window.single_mut();
    twindow.cursor_options = bevy::window::CursorOptions {
        visible: false,
        grab_mode: CursorGrabMode::Locked,

        ..Default::default()
    };

    for mut controller in controller_query.iter_mut() {
        controller.enable_input = true;
    }

    println!("Exiting Main Menu, Moving to In Game");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
