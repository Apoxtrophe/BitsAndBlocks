use bevy::prelude::*;
use crate::prelude::*;

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