use bevy::prelude::*;
use crate::prelude::*;

pub fn spawn_new_game_ui(
    mut commands: &mut Commands,
    image_handles: &Res<GameTextures>,
    button_handle: &Handle<Image>,
    atlas_handle: &Handle<TextureAtlasLayout>,
) -> Entity {
    let image_handlezzz = image_handles.new_game_screen_texture.clone();
    let new_game_main = spawn_popup(&mut commands, image_handlezzz, GameUI::NewGame);
    let new_game_sub = spawn_sub_node(&mut commands, 100.0, 15.0, 10.0);
    
    let button_options = [
      MenuAction::CreateWorld,
      ].to_vec();
    
    for i in 0..button_options.len() {
        spawn_button(
            &mut commands,
            new_game_sub,
            button_handle.clone(),
            atlas_handle.clone(),
            button_options[i].clone(),
            100.0,
        );
    }
    
    let editable_text = create_editable_text(&mut commands);
    
    commands.entity(editable_text).set_parent(new_game_main);
    commands.entity(new_game_sub).set_parent(new_game_main);
    
    new_game_main
}
