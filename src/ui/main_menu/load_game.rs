use crate::prelude::*;
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::focus::HoverMap,
};
use bevy::prelude::*;

pub fn spawn_load_game_ui(
    mut commands: &mut Commands,
    image_handles: &Res<GameTextures>,
    names_list: &Vec<String>,
) -> Entity {
    let load_game_window = spawn_popup(
        &mut commands,
        image_handles.load_game_screen_texture.clone(),
        GameUI::LoadGame,
    );
    //let load_game_sub = spawn_sub_node(&mut commands, 50.0, 40.0, 20.0);

    let scrollable_list = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                height: Val::Percent(60.),
                top: Val::Percent(30.0),
                width: Val::Percent(70.),
                row_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Percent(1.5)),
                justify_self: JustifySelf::Center,
                justify_content: JustifyContent::Center,
                overflow: Overflow::scroll_y(), // n.b.
                ..default()
            },
            BackgroundColor(Color::linear_rgb(0.1, 0.1, 0.1)),
        ))
        .id();

    // Load Game Buttons

    for i in 0..names_list.len() {
        // Create an outer container for each row.
        let row_container = spawn_ui_node(
            &mut commands,
            Node {
                width: Val::Percent(100.0),
                // Arrange items horizontally.
                flex_direction: FlexDirection::Row,
                // Align items in the center vertically.
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                // You can also set a margin for the container itself if needed.
                //margin: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            (),
        );
    
        // Spawn the world button and attach it to the row container.
        let world_button = spawn_text_button(
            &mut commands,
            100.0,
            100.0,
            names_list[i].clone(),
            MenuAction::LoadWorld(names_list[i].clone()),
        );
        commands.entity(world_button).set_parent(row_container);
    
        // Spawn the delete button.
        let delete_button = spawn_text_button(
            &mut commands,
            30.0,
            100.0,
            " X ".to_string(),
            MenuAction::DeleteWorld(names_list[i].clone()),
        );
        // Add a left margin to separate it from the world button.
        commands.entity(delete_button).insert(Node {
            margin: UiRect {
                left: Val::Px(10.0),
                ..Default::default()
            },
            ..default()
        });
        // Instead of setting the delete button as a child of world_button,
        // set it as a sibling within the row container.
        commands.entity(delete_button).set_parent(row_container);
    
        // Attach the entire row container to your scrollable list.
        commands.entity(row_container).set_parent(scrollable_list);
    }


    commands
        .entity(scrollable_list)
        .set_parent(load_game_window);

    load_game_window
}

/// Updates the scroll position of scrollable nodes in response to mouse input
pub fn update_scroll_position(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let (mut dx, mut dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (mouse_wheel_event.x * 34.0, mouse_wheel_event.y * 34.0),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };

        if keyboard_input.pressed(KeyCode::ControlLeft)
            || keyboard_input.pressed(KeyCode::ControlRight)
        {
            std::mem::swap(&mut dx, &mut dy);
        }

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
