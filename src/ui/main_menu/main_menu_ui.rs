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

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn menu_button_system(
    mut query: Query<(&Interaction, &mut BackgroundColor, &MenuButton), Changed<Interaction>>,
    mut app_state: ResMut<NextState<GameState>>,
    mut game_ui: ResMut<GameUI>,
    mut game_save: ResMut<SavedWorld>,
    // other resources like Commands, VoxelMap, Meshes, etc.
    mut exit: EventWriter<AppExit>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    events: EventReader<TextInputSubmitEvent>, // Should probably be moved into the Event Handler
    mut event_writer: EventWriter<GameEvent>,
    mut commands: Commands,
    mut voxel_map: ResMut<VoxelMap>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (interaction, mut bg_color, menu_button) in query.iter_mut() {
        // Update button color with our helper.
        update_bg_color(interaction, &mut bg_color);
        if let Interaction::Pressed = *interaction {
            match &menu_button.action {
                MenuAction::LoadWorld(name) => {
                    println!("WORLD NAME: {}", name);
                    if name.is_empty() {
                        println!("Empty Slot");
                    } else {
                        println!("Load Game: {}", name);
                        load_world(name, &mut commands, &mut voxel_map, &mut meshes);
                        game_save.world_name = name.clone();
                        *game_ui = GameUI::Default;
                        app_state.set(GameState::InGame);
                    }
                }
                MenuAction::NewGame => {
                    *game_ui = GameUI::NewGame;
                }
                MenuAction::LoadGame => {
                    *game_ui = GameUI::LoadGame;
                }
                MenuAction::Options => {
                    *game_ui = GameUI::Options;
                }
                MenuAction::QuitGame => {
                    exit.send(AppExit::Success);
                }
                MenuAction::CreateWorld => {
                    println!("Create World {}", game_save.world_name);
                    if game_save.world_name.len() > 0 {
                        *game_ui = GameUI::Default;
                        app_state.set(GameState::InGame);
                    } else {
                        *bg_color = Color::linear_rgba(1.0, 0.0, 0.0, 1.0).into();
                    }
                }
                MenuAction::BackToGame => {
                    *game_ui = GameUI::Default;
                    event_writer.send(GameEvent::UpdateCursor {
                        mode: CursorGrabMode::Locked,
                        show_cursor: false,
                        enable_input: true,
                    });
                }
                MenuAction::MainMenu => {
                    event_writer.send(GameEvent::SaveWorld {
                        world: game_save.clone(),
                    });

                    event_writer.send(GameEvent::StateChange {
                        new_state: GameState::Loading,
                    });
                }
                MenuAction::SaveAndQuit => {
                    event_writer.send(GameEvent::SaveWorld {
                        world: game_save.clone(),
                    });
                    exit.send(AppExit::Success);
                }
                MenuAction::Placeholder => {
                }
                // Handle other actions if needed.
            }
        }
    }
    
    // Shitty code for handling escape in the main menu
    if keyboard_input.just_pressed(KeyCode::Escape) 
    && *game_ui != GameUI::Default
    && *game_ui != GameUI::Inventory
    && *game_ui != GameUI::ExitMenu
    {
        *game_ui = GameUI::MainScreen;
    }
    
    edit_text_listener(events, game_save);
}
fn update_bg_color(interaction: &Interaction, bg_color: &mut BackgroundColor) {
    *bg_color = match *interaction {
        Interaction::Pressed => Color::linear_rgba(0.0, 1.0, 0.0, 1.0).into(),
        Interaction::Hovered => Color::linear_rgba(1.0, 1.0, 1.0, 1.0).into(),
        Interaction::None => Color::linear_rgba(0.5, 0.5, 0.5, 0.25).into(),
    };
}


