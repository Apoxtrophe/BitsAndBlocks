use crate::prelude::*;
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::focus::HoverMap,
};

pub fn spawn_load_game_ui(
    commands: &mut Commands,
    image_handles: &Res<GameTextures>,
    names_list: &Vec<String>,
) -> Entity {
    // Spawn the main load game popup.
    let load_game_window = spawn_popup(
        commands,
        image_handles.load_game_screen_texture.clone(),
        GameUI::LoadGame,
    );

    // Create a scrollable list node.
    let scrollable_list = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                row_gap: Val::Percent(2.0),
                top: Val::Percent(8.0),
                padding: UiRect::all(Val::Percent(10.0)),
                justify_self: JustifySelf::Center,
                // Align items to the top so nothing is cut off.
                justify_content: JustifyContent::FlexStart,
                overflow: Overflow::scroll_y(),
                ..default()
            },
            // Uncomment the following if you need a background color:
            // BackgroundColor(Color::linear_rgb(0.1, 0.1, 0.1)),
        ))
        .id();

    // Spawn a row for each game save name.
    for name in names_list.iter() {
        // Create a row container for the load and delete buttons.
        let row_container = commands
            .spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart, // Using FlexStart to group items together.
                    ..default()
                },
            ))
            .id();
        commands.entity(row_container).set_parent(scrollable_list);

        // Spawn the world load button.
        let load_button = spawn_text_button(
            commands,
            80.0,
            100.0,
            name.clone(),
            MenuAction::LoadWorld(name.clone()),
        );
        commands.entity(load_button).set_parent(row_container);

        // Spawn the delete button.
        let delete_button = spawn_text_button(
            commands,
            30.0,
            100.0,
            " X ".to_string(),
            MenuAction::DeleteWorld(name.clone()),
        );
        commands.entity(delete_button).set_parent(row_container);
    }

    // Attach the scrollable list to the load game window.
    commands.entity(scrollable_list).set_parent(load_game_window);

    load_game_window
}

/// Updates the scroll position of scrollable nodes in response to mouse input.
pub fn update_scroll_position(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for event in mouse_wheel_events.read() {
        // Convert scroll units to pixels.
        let (mut dx, mut dy) = match event.unit {
            MouseScrollUnit::Line => (event.x * 34.0, event.y * 34.0),
            MouseScrollUnit::Pixel => (event.x, event.y),
        };

        // Swap scroll direction if a Control key is held down.
        if keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ControlRight) {
            std::mem::swap(&mut dx, &mut dy);
        }

        // Apply scroll offset to any hovered scrollable node.
        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.offset_x -= dx;
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}
