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
               //BackgroundColor(Color::linear_rgb(0.1, 0.1, 0.1)),
           ))
           .id();

    // Load Game Buttons

    for i in 0..names_list.len() {
        // Create a row container for the two buttons.
        let row_container = commands
            .spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart, // Change from SpaceBetween
                    ..default()
                },
            ))
            .id();
        // Attach the row container to the scrollable list.
        commands.entity(row_container).set_parent(scrollable_list);
    
        // Spawn the world load button and add it to the row container.
        let world_button = spawn_text_button(
            &mut commands,
            80.0,
            100.0,
            names_list[i].clone(),
            MenuAction::LoadWorld(names_list[i].clone()),
        );
        commands.entity(world_button).set_parent(row_container);
    
        // Spawn the delete button and add it to the row container.
        let delete_button = spawn_text_button(
            &mut commands,
            30.0,
            100.0,
            " X ".to_string(),
            MenuAction::DeleteWorld(names_list[i].clone()),
        );
        commands.entity(delete_button).set_parent(row_container);
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
