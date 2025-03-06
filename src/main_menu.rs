use bevy::prelude::*;
use crate::{loading::GameTextures, ui_helpers::{create_atlas_image, spawn_ui_node}, GameState};

#[derive(Component)]
pub struct MainMenuEntity;

#[derive(Component)]
pub struct ButtonNumber {
    index: usize,
}

#[derive(Component, Debug)]
pub struct PopUp {
    screen_type: CurrentScreen,
}

// Temporary Resource
#[derive(Resource, Debug)]
pub struct WhichScreen {
    pub screen: CurrentScreen,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CurrentScreen {
    MainScreen,
    NewGame,
    LoadGame,
    Settings,
}

pub fn setup_main_menu(
    mut commands: Commands,
    image_handles: Res<GameTextures>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    println!("State: Main Menu");

    commands.insert_resource(WhichScreen {
        screen: CurrentScreen::MainScreen,
    });

    // Spawn the camera tagged for the main menu
    commands.spawn(Camera2d).insert(MainMenuEntity);

    // Spawn the main menu background node (fills the screen)
    let root_node = spawn_root_node(&mut commands);
    // Spawn the main screen node with a fixed 16:9 aspect ratio

    let main_menu = spawn_popup(&mut commands, image_handles.home_screen_texture.clone(), CurrentScreen::MainScreen);
    commands.entity(main_menu).set_parent(root_node);
    
        let main_menu_sub = spawn_sub_node(&mut commands, 40.0, 60.0, 10.0);
        commands.entity(main_menu_sub).set_parent(main_menu);
    
    // Spawn the popups
    let new_game = spawn_popup(&mut commands, image_handles.new_game_screen_texture.clone(), CurrentScreen::NewGame);
    commands.entity(new_game).set_parent(root_node);
    
        let new_game_sub = spawn_sub_node(&mut commands, 100.0, 15.0, 10.0);
        commands.entity(new_game_sub.clone()).set_parent(new_game);
    
    let load_game = spawn_popup(&mut commands, image_handles.load_game_screen_texture.clone(), CurrentScreen::LoadGame);
    commands.entity(load_game).set_parent(root_node);
    
    let options_screen = spawn_popup(&mut commands, image_handles.options_screen_texture.clone(), CurrentScreen::Settings);
    commands.entity(options_screen).set_parent(root_node);
    
    
    // Prepare button texture atlas
    let buttons_texture = image_handles.menu_button_texture.clone();
    let button_atlas = TextureAtlasLayout::from_grid(UVec2::new(144, 32), 1, 8, None, None);
    let button_atlas_handle = texture_atlases.add(button_atlas);

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
    
    // New Game Buttons
    for i in 4..5 {
        spawn_button(
            &mut commands,
            new_game_sub,
            buttons_texture.clone(),
            button_atlas_handle.clone(),
            i,
            100.0,
        );
    }
}

/// Spawns the root container node that fills the screen and has a black background.
fn spawn_root_node(commands: &mut Commands) -> Entity {
    spawn_ui_node(
        commands,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::Center,
            ..default()
        },
        (BackgroundColor(Color::BLACK), MainMenuEntity),
    )
}

fn spawn_popup(
    commands: &mut Commands,
    image_handle: Handle<Image>,
    screen_type: CurrentScreen,
) -> Entity {
    spawn_ui_node(
        commands,
        Node {
            width: Val::Auto,
            height: Val::Percent(100.0),
            aspect_ratio: Some(16.0 / 9.0),
            flex_wrap: FlexWrap::Wrap,
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            ..default()
        },
        (ImageNode::new(image_handle), Visibility::Hidden, PopUp { screen_type }),
    )
}

/// Creates a sub-node that can be attached as a child (for instance, to hold extra buttons).
fn spawn_sub_node(commands: &mut Commands, width: f32, height: f32, bottom: f32) -> Entity {
    spawn_ui_node(
        commands,
        Node {
            width: Val::Percent(width),
            height: Val::Percent(height),
            bottom: Val::Percent(bottom),
            align_content: AlignContent::Center,
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
        (), // No extra components
    )
}

/// Spawns a button with a container (acting as the border) and an image child.
/// The container includes a BackgroundColor that will update based on user interaction.
/// Spawns a button consisting of a container and an image child.
/// The button container is spawned with default styling, then an image node is attached.
fn spawn_button(
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


/// System to update button colors based on user interaction.
/// Add this system to your Update schedule.
pub fn menu_interaction_system(
    mut query: Query<(&Interaction, &mut BackgroundColor, &ButtonNumber), (Changed<Interaction>, With<Button>)>,
    mut current_screen: ResMut<WhichScreen>,
    mut exit: EventWriter<AppExit>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut bg_color, button_number) in query.iter_mut() {
        match *interaction {
            
            Interaction::Pressed => {                
                match button_number.index {
                    0 => {
                        println!("New Game");
                        current_screen.screen = CurrentScreen::NewGame;
                    }
                    1 => {
                        println!("Load Game");
                        current_screen.screen = CurrentScreen::LoadGame;
                    }
                    2 => {
                        println!("Options");
                        current_screen.screen = CurrentScreen::Settings;
                    }
                    3 => {
                        println!("Quit Game");
                        exit.send(AppExit::Success);
                    }
                    4 => {
                        println!("Create World");
                        app_state.set(GameState::InGame);
                    }
                    _ => {}
                }

                *bg_color = Color::linear_rgba(0.0, 1.0, 0.0, 1.0).into();
            }
            Interaction::Hovered => {
                *bg_color = Color::linear_rgba(1.0, 1.0, 1.0, 1.0).into();
            }
            Interaction::None => {
                *bg_color = Color::linear_rgba(0.0, 0.0, 0.0, 0.0).into();
            }
        }
    }
    
    if keyboard_input.just_pressed(KeyCode::Escape) {
        println!("Main Menu");
        current_screen.screen = CurrentScreen::MainScreen;
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
) {
    println!("Exiting Main Menu, Moving to In Game");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<WhichScreen>();
}

