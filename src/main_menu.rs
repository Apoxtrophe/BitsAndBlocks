use bevy::prelude::*;
use crate::{loading::GameTextures, GameState};

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
    Main_Screen,
    New_Game,
    Load_Game,
    Settings,
}

pub fn setup_main_menu(
    mut app_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    image_handles: Res<GameTextures>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    println!("State: Main Menu");

    commands.insert_resource(WhichScreen {
        screen: CurrentScreen::Main_Screen,
    });
    
    // 
    
    // Spawn the camera tagged for the main menu
    commands.spawn(Camera2d).insert(MainMenuEntity);

    // Spawn the main menu background node (fills the screen)
    let main_menu = spawn_main_menu_node(&mut commands);
    // Spawn the main screen node with a fixed 16:9 aspect ratio
    let main_screen = spawn_main_screen_node(&mut commands, image_handles.home_screen_texture.clone());
    commands.entity(main_screen).set_parent(main_menu);
    // Spawn the options container node as a child of the main screen
    let options = spawn_options_node(&mut commands);
    commands.entity(options).set_parent(main_screen);
    
    //
    // POPUPS
    //
    // Sub node that holds each's respective buttons n such
    let sub_node = create_sub_node(&mut commands);
    
    
    // Spawn the new game window
    let new_game = spawn_new_game(&mut commands, image_handles.new_game_screen_texture.clone());
    commands.entity(new_game).set_parent(main_screen);
    commands.entity(sub_node.clone()).set_parent(new_game);
    
    let options_screen = spawn_options_screen(&mut commands, image_handles.options_screen_texture.clone());
    commands.entity(options_screen).set_parent(main_screen);
    
    let load_game = spawn_load_game(&mut commands, image_handles.load_game_screen_texture.clone());
    commands.entity(load_game).set_parent(main_screen);
    

    //
    // POPUPS
    //
    
    // Prepare button texture atlas
    let buttons_texture = image_handles.menu_button_texture.clone();
    let button_atlas = TextureAtlasLayout::from_grid(UVec2::new(144, 32), 1, 8, None, None);
    let button_atlas_handle = texture_atlases.add(button_atlas);

    // Spawn four buttons
    for i in 0..4 {
        spawn_button(
            &mut commands,
            options,
            buttons_texture.clone(),
            button_atlas_handle.clone(),
            i,
            24.0,
        );
        println!("{}", i)
    }
    
    for i in 4..5 {
        spawn_button(
            &mut commands,
            sub_node,
            buttons_texture.clone(),
            button_atlas_handle.clone(),
            i,
            100.0,
        );
    }
    // Optionally update state:
    // app_state.set(GameState::InGame);
}

fn spawn_main_menu_node(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            // Full-screen container; its background remains black.
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::BLACK),
            MainMenuEntity,
        ))
        .id()
}

/// Modified to force a 16:9 aspect ratio by letting the height drive the layout.
/// The width is set to auto and horizontal margins are auto to center the content.
fn spawn_main_screen_node(commands: &mut Commands, home_screen_handle: Handle<Image>) -> Entity {
    commands
        .spawn((
            Node {
                // Use automatic width so the aspect_ratio takes over
                width: Val::Auto,
                // Use full height of the parent (with some padding)
                height: Val::Percent(100.0),
                // Enforce a 16:9 aspect ratio, so the width is computed from the height
                aspect_ratio: Some(16.0 / 9.0),
                // Center horizontally, leaving black bars on ultrawide displays
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    ..default()
                },
                padding: UiRect::all(Val::Px(30.0)),
                column_gap: Val::Px(30.0),
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ImageNode::new(home_screen_handle),
        ))
        .id()
}

fn spawn_options_node(commands: &mut Commands) -> Entity {
    let options_node = (Node {
            // These percentages now relate to the fixed-aspect main screen
            width: Val::Percent(40.0),
            height: Val::Percent(60.0),
            top: Val::Percent(35.0),
            aspect_ratio: Some(144.0 / 32.0),
            flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::Center,
            ..default()
    },
    PopUp{screen_type:CurrentScreen::Main_Screen}
    );
    
    commands.spawn(options_node).id()
}

fn create_sub_node(
    commands: &mut Commands,
) -> Entity {
    let sub_node = (Node {
        width: Val::Percent(100.0),
        height: Val::Percent(15.0),
        bottom: Val::Percent(10.0),
        align_content: AlignContent::Center,
        position_type: PositionType::Absolute,
        justify_content: JustifyContent::Center,
        ..Default::default()
    });
    
    commands.spawn(sub_node).id()
}

fn spawn_new_game(
    commands: &mut Commands,
    game_screen: Handle<Image>,
) -> Entity {
    let new_game_node = (Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_self: AlignSelf::Center,
        justify_self: JustifySelf::Center,
        align_content: AlignContent::Center,
        justify_content: JustifyContent::Center,
        flex_wrap: FlexWrap::Wrap,
        position_type: PositionType::Absolute,
        ..Default::default()
    },
    ImageNode::new(game_screen),
    Visibility::Hidden,
    PopUp{screen_type:CurrentScreen::New_Game});
    
    commands.spawn(new_game_node).id()
}

fn spawn_load_game(
    commands: &mut Commands,
    game_screen: Handle<Image>,
) -> Entity {
    let new_game_node = (Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_self: AlignSelf::Center,
        justify_self: JustifySelf::Center,
        flex_wrap: FlexWrap::Wrap,
        position_type: PositionType::Absolute,
        ..Default::default()
    },
    ImageNode::new(game_screen),
    Visibility::Hidden,
    PopUp{screen_type:CurrentScreen::Load_Game});
    
    commands.spawn(new_game_node).id()
}

fn spawn_options_screen(
    commands: &mut Commands,
    game_screen: Handle<Image>,
) -> Entity {
    let new_game_node = (Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_self: AlignSelf::Center,
        justify_self: JustifySelf::Center,
        flex_wrap: FlexWrap::Wrap,
        position_type: PositionType::Absolute,
        ..Default::default()
    },
    ImageNode::new(game_screen),
    Visibility::Hidden,
    PopUp{screen_type:CurrentScreen::Settings});
    
    commands.spawn(new_game_node).id()
}

/// Spawns a button with a container (acting as the border) and an image child.
/// The container includes a BackgroundColor that will update based on user interaction.
fn spawn_button(
    commands: &mut Commands,
    parent: Entity,
    texture_handle: Handle<Image>,
    button_atlas_handle: Handle<TextureAtlasLayout>,
    index: usize,
    height_percent: f32,
) -> Entity {
    // Button container with a default white border color.
    let button_container = commands
        .spawn((
            Button,
            Node {
                width: Val::Auto,
                height: Val::Percent(height_percent),
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::WHITE),
            ButtonNumber { index },
        ))
        .id();
    commands.entity(button_container).set_parent(parent);

    // Create the image node and update the atlas index for this button.
    let mut image_node = ImageNode::from_atlas_image(texture_handle, TextureAtlas::from(button_atlas_handle));
    if let Some(atlas) = &mut image_node.texture_atlas {
        atlas.index = index;
    }

    // Spawn the image node with a margin so the container’s background (border) remains visible.
    let image_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(90.0),
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                margin: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            image_node,
        ))
        .id();
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
                        current_screen.screen = CurrentScreen::New_Game;
                    }
                    1 => {
                        println!("Load Game");
                        current_screen.screen = CurrentScreen::Load_Game;
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
        current_screen.screen = CurrentScreen::Main_Screen;
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

/*
if pop_num.index == 0 {
    match current_screen.screen {
        CurrentScreen::New_Game => {
            *visibility = Visibility::Visible;
        }
        _ => {
            *visibility = Visibility::Hidden;
        }
    }
}
*/

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
