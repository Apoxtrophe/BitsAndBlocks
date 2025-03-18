use bevy::prelude::*;
use crate::prelude::*;

pub fn spawn_options_ui(
    commands: &mut Commands,
    image_handles: &Res<GameTextures>,
) -> Entity{
    let options_node = spawn_popup(commands, image_handles.options_screen_texture.clone(), WhichMenuUI::Options);
    options_node
}
