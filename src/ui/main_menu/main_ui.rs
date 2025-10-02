use bevy::prelude::*;
use crate::prelude::*;

pub fn spawn_main_ui(
    commands: &mut Commands,
    image_handles: &Res<GameTextures>,
    button_handle: &Handle<Image>,
    atlas_handle: &Handle<TextureAtlasLayout>,
) -> Entity {
    // Create root node and main popup.
    let root_ui = spawn_root_node(commands);
    let main_ui = spawn_popup(commands, image_handles.home_screen_texture.clone(), GameUI::MainScreen);
    commands.entity(main_ui).set_parent(root_ui);

    // Create a sub-node for the main menu buttons.
    let main_menu_sub = spawn_ui_node(
            commands,
            Node {
                width: Val::Percent(40.0),
                height: Val::Percent(60.0),
                bottom: Val::Percent(20.0),
                right: Val::Percent(6.0),
                row_gap: Val::Px(8.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            (),
        );
    commands.entity(main_menu_sub).set_parent(main_ui);

    // Define the button actions for the main menu.
    let button_options = vec![
        MenuAction::NewGame,
        MenuAction::LoadGame,
        MenuAction::Options,
        MenuAction::QuitGame,
    ];

    // Spawn each button using an iterator.
    for action in button_options {
        spawn_button(
            commands,
            main_menu_sub,
            button_handle.clone(),
            atlas_handle.clone(),
            action,
            24.0,
        );
    }

    root_ui
}