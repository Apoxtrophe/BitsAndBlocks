use bevy::prelude::*;
use crate::prelude::*;

pub fn spawn_new_game_ui(
    mut commands: &mut Commands,
    image_handles: &Res<GameTextures>,
    image_handle: &Handle<Image>,
    atlas_handle: &Handle<TextureAtlasLayout>,
    button_pointer: (usize, usize), // Starting index and number of buttons
) -> Entity {
    let image_handlezzz = image_handles.new_game_screen_texture.clone();
    let new_game_main = spawn_popup(&mut commands, image_handlezzz, WhichMenuUI::NewGame);
    let new_game_sub = spawn_sub_node(&mut commands, 100.0, 15.0, 10.0);
    
    
    let (start, end) = (button_pointer.0, button_pointer.0 + button_pointer.1);
    // New Game Buttons
    for i in start..end {
        spawn_button(
            &mut commands,
            new_game_sub,
            image_handle.clone(),
            atlas_handle.clone(),
            i,
            100.0,
        );
    }
    
    let editable_text = create_editable_text(&mut commands);
    
    commands.entity(editable_text).set_parent(new_game_main);
    commands.entity(new_game_sub).set_parent(new_game_main);
    
    new_game_main
}
