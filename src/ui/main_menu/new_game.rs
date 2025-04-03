use bevy::prelude::*;
use crate::prelude::*;

pub fn spawn_new_game_ui(
    commands: &mut Commands,
    image_handles: &Res<GameTextures>,
    button_handle: &Handle<Image>,
    atlas_handle: &Handle<TextureAtlasLayout>,
) -> Entity {
    // Use a clear variable name for the new game texture.
    let new_game_texture = image_handles.new_game_screen_texture.clone();
    let new_game_main = spawn_popup(commands, new_game_texture, GameUI::NewGame);
    let new_game_sub = spawn_sub_node(commands, 100.0, 15.0, 10.0);

    // Define the button actions for the new game UI.
    let button_options = vec![MenuAction::CreateWorld];

    // Spawn the button(s) for creating a new world.
    for action in button_options {
        spawn_button(
            commands,
            new_game_sub,
            button_handle.clone(),
            atlas_handle.clone(),
            action,
            100.0,
        );
    }

    // Create an editable text element and attach it.
    let editable_text = create_editable_text(commands);
    commands.entity(editable_text).set_parent(new_game_main);
    commands.entity(new_game_sub).set_parent(new_game_main);

    new_game_main
}
