use bevy::{input::mouse::{MouseScrollUnit, MouseWheel}, picking::focus::HoverMap, prelude::*};
use crate::prelude::*;

pub fn spawn_load_game_ui(
    mut commands: &mut Commands,
    image_handles: &Res<GameTextures>,
    button_pointer: (usize, usize), // Starting index and number of buttons
    names_list: &Vec<String>,
) -> Entity{
    let load_game_window = spawn_popup(&mut commands, image_handles.load_game_screen_texture.clone(), WhichMenuUI::LoadGame);
    //let load_game_sub = spawn_sub_node(&mut commands, 50.0, 40.0, 20.0);
    
    let scrollable_list = commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            align_self: AlignSelf::Stretch,
            height: Val::Percent(60.),
            top: Val::Percent(30.0),
            width: Val::Percent(60.),
            row_gap: Val::Px(8.0),
            padding: UiRect::all(Val::Percent(1.5)),
            justify_self: JustifySelf::Center,
            overflow: Overflow::scroll_y(), // n.b.
            ..default()
        },BackgroundColor(Color::linear_rgb(0.1, 0.1, 0.1)))).id();
    
    
    // Load world names as Options (They may not exist)
    /*
    let mut names = Vec::new();
    for i in 0..names_list.len() {
        if names_list[i].is_some() {
            names.push(names_list[i].clone().unwrap());
        } else {
            let slot_name = format!("empty");
            names.push(slot_name);
        }
    }
    */

    // Load Game Buttons
    let (start, end) = (button_pointer.0, button_pointer.0 + names_list.len());
    
    
    for i in start..end {
        spawn_text_button(
            &mut commands,
            scrollable_list,
            50.0,
            15.0,
            i,
            names_list[i - start].clone(),
        );
    }
    
    commands.entity(scrollable_list).set_parent(load_game_window);
    
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
            MouseScrollUnit::Line => (
                mouse_wheel_event.x * 34.0,
                mouse_wheel_event.y * 34.0
            ),
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