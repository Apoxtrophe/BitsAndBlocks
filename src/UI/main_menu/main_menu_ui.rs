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

    commands.insert_resource(WhichScreen {
        screen: WhichMenuUI::MainScreen,
    });

    // Spawn the camera tagged for the main menu
    commands.spawn(Camera2d).insert(MainMenuEntity);

    // Spawn the main menu background node (fills the screen)
    let root_node = spawn_root_node(&mut commands);
    // Spawn the main screen node with a fixed 16:9 aspect ratio

    let main_menu = spawn_popup(&mut commands, image_handles.home_screen_texture.clone(), WhichMenuUI::MainScreen);
    commands.entity(main_menu).set_parent(root_node);
    
        let main_menu_sub = spawn_sub_node(&mut commands, 40.0, 60.0, 10.0);
        commands.entity(main_menu_sub).set_parent(main_menu);
    
    
    // Prepare button texture atlas
    let buttons_texture = image_handles.menu_button_texture.clone();
    let button_atlas = TextureAtlasLayout::from_grid(UVec2::new(144, 32), 1, 16, None, None);
    let button_atlas_handle = texture_atlases.add(button_atlas);
    
    // Spawn the New Game Window
    let new_game_window = spawn_new_game_ui(&mut commands, &image_handles, &buttons_texture, &button_atlas_handle, (4,1));
    commands.entity(new_game_window).set_parent(root_node);
    
    // Spawn the Load Game Window
    let load_game_window = spawn_load_game_ui(&mut commands, &image_handles, (0,6), &saved_games.saves);
    commands.entity(load_game_window).set_parent(root_node);
        
    
    let options_screen = spawn_popup(&mut commands, image_handles.options_screen_texture.clone(), WhichMenuUI::Settings);
    commands.entity(options_screen).set_parent(root_node);
    
    // Spawn four buttons
    for i in 0..4 {
        spawn_button(
            &mut commands,
            main_menu_sub,
            buttons_texture.clone(),
            button_atlas_handle.clone(),
            i,
            24.0,
        );
    }
}

/*

let load_game = spawn_popup(&mut commands, image_handles.load_game_screen_texture.clone(), WhichMenuUI::LoadGame);
commands.entity(load_game).set_parent(root_node);
    
    let load_game_sub = spawn_sub_node(&mut commands, 50.0, 40.0, 20.0);
    commands.entity(load_game_sub).set_parent(load_game);
*/

pub fn spawn_load_game_ui(
    mut commands: &mut Commands,
    image_handles: &Res<GameTextures>,
    button_pointer: (usize, usize), // Starting index and number of buttons
    names_list: &Vec<Option<String>>,
) -> Entity{
    let load_game_window = spawn_popup(&mut commands, image_handles.load_game_screen_texture.clone(), WhichMenuUI::LoadGame);
    let load_game_sub = spawn_sub_node(&mut commands, 50.0, 40.0, 20.0);
    
    // Load world names as Options (They may not exist)
    let mut names = Vec::new();
    for i in 0..names_list.len() {
        if names_list[i].is_some() {
            names.push(names_list[i].clone().unwrap());
        } else {
            let slot_name = format!("empty");
            names.push(slot_name);
        }
    }
    // Load Game Buttons
    let (start, end) = (button_pointer.0, button_pointer.0 + button_pointer.1);
    
    for i in start..end {
        spawn_text_button(
            &mut commands,
            load_game_sub,
            50.0,
            15.0,
            i,
            names[i].clone(),
        );
    }
    
    commands.entity(load_game_sub).set_parent(load_game_window);
    
    load_game_window
}

/// Spawns a button with a container (acting as the border) and an image child.
/// The container includes a BackgroundColor that will update based on user interaction.
/// Spawns a button consisting of a container and an image child.
/// The button container is spawned with default styling, then an image node is attached.
pub fn spawn_button(
    commands: &mut Commands,
    parent: Entity,
    texture_handle: Handle<Image>,
    button_atlas_handle: Handle<TextureAtlasLayout>,
    index: usize,
    height_percent: f32,
) -> Entity {
    // Spawn the button container with a white background.
    let button_container = spawn_ui_node(
        commands,
        Node {
            width: Val::Auto,
            height: Val::Percent(height_percent),
            justify_content: JustifyContent::Center,
            ..default()
        },
        (Button, BackgroundColor(Color::WHITE), ButtonNumber { index }),
    );
    commands.entity(button_container).set_parent(parent);

    // Spawn the child image node with a margin so the container's border shows.
    let image_entity = spawn_ui_node(
        commands,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(90.0),
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            margin: UiRect::all(Val::Px(4.0)),
            ..default()
        },
        create_atlas_image(texture_handle, button_atlas_handle, index),
    );
    commands.entity(image_entity).set_parent(button_container);

    button_container
}

#[derive(Component)]
pub struct WorldButton {
    pub index: usize,
    name: String,
}

fn spawn_text_button(
    commands: &mut Commands,
    parent: Entity,
    weight: f32,
    height: f32,
    button_index: usize,
    text: String,
) {
    let button_container = spawn_ui_node(
        commands,
        Node {
            width: Val::Percent(weight),
            height: Val::Percent(height),
            justify_content: JustifyContent::Center,
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            margin: UiRect::all(Val::Px(4.0)),
            ..default()
        },
        (Button, BackgroundColor(Color::WHITE), WorldButton { index: button_index, name: text.clone() }),
    );
    
    commands.entity(button_container).set_parent(parent);
    
    let text_node = commands.spawn((
        Text::new(text),
        TextFont {
            font_size: 32.0, 
            ..Default::default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            justify_content: JustifyContent::Center,
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            ..Default::default()
        },
    )).id();
    
    commands.entity(text_node).set_parent(button_container);
}


/// System to update button colors based on user interaction.
/// Add this system to your Update schedule.
pub fn menu_interaction_system(
    mut query: Query<(&Interaction, &mut BackgroundColor, &ButtonNumber), (Changed<Interaction>, With<Button>)>,
    mut current_screen: ResMut<WhichScreen>,
    mut exit: EventWriter<AppExit>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<GameState>>,
    save_world: ResMut<SavedWorld>,
    events: EventReader<TextInputSubmitEvent>,
) {

    for (interaction, mut bg_color, button_number) in query.iter_mut() {
        match *interaction {
            
            Interaction::Pressed => {                
                match button_number.index {
                    0 => {
                        println!("New Game");
                        current_screen.screen = WhichMenuUI::NewGame;
                    }
                    1 => {
                        println!("Load Game");
                        current_screen.screen = WhichMenuUI::LoadGame;
                    }
                    2 => {
                        println!("Options");
                        current_screen.screen = WhichMenuUI::Settings;
                    }
                    3 => {
                        println!("Quit Game");
                        exit.send(AppExit::Success);
                    }
                    4 => {
                        println!("Create World");
                        if save_world.world_name.len() > 0 {
                            app_state.set(GameState::InGame);
                        }
                    }
                    _ => {}
                }

                *bg_color = Color::linear_rgba(0.0, 1.0, 0.0, 1.0).into();
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
        current_screen.screen = WhichMenuUI::MainScreen;
    }
    
    edit_text_listener(events, save_world);
}

pub fn world_button_system(
    mut world_button_query: Query<(&Interaction, &mut BackgroundColor, &WorldButton), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<GameState>>,
    
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
                    "empty" => {
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
        app_state.set(GameState::InGame);
    }
}

pub fn update_pop_window_visibility(

    mut query: Query<(&PopUp, &mut Visibility)>,
    current_screen: Res<WhichScreen>,
) {
    for (popup, mut visibility) in query.iter_mut() {
        if popup.screen_type == current_screen.screen {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
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
    commands.remove_resource::<WhichScreen>();
}

