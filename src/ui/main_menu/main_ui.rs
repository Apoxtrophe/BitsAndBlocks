use bevy::prelude::*;
use crate::prelude::*;

pub fn spawn_main_ui (
    commands: &mut Commands,
    image_handles: &Res<GameTextures>,
    button_handle: &Handle<Image>,
    atlas_handle: &Handle<TextureAtlasLayout>,
    button_pointer: (usize, usize), // Starting index and number of buttons
) -> Entity {
    let root_ui = spawn_root_node(commands);
    let main_ui = spawn_popup(commands, image_handles.home_screen_texture.clone(), WhichMenuUI::MainScreen);
    commands.entity(main_ui).set_parent(root_ui);
    let main_menu_sub = spawn_sub_node(commands, 40.0, 60.0, 10.0);
    commands.entity(main_menu_sub).set_parent(main_ui);
    
    let (start, end) = (button_pointer.0, button_pointer.0 + button_pointer.1);
    
    // Spawn four buttons
    for i in start..end {
        spawn_button(
            commands,
            main_menu_sub,
            button_handle.clone(),
            atlas_handle.clone(),
            i,
            24.0,
        );
    }
    
    root_ui
}