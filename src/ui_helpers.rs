use bevy::prelude::*;

/// A generic helper that spawns a UI node with a given style and additional components.
pub fn spawn_ui_node<B: Bundle>(commands: &mut Commands, style: Node, bundle: B) -> Entity {
    commands.spawn((style, bundle)).id()
}

/// A helper function that creates an ImageNode from an atlas image and sets the atlas index.
pub fn create_atlas_image(
    texture_handle: Handle<Image>,
    button_atlas_handle: Handle<TextureAtlasLayout>,
    index: usize,
) -> ImageNode {
    let mut image_node = ImageNode::from_atlas_image(texture_handle, TextureAtlas::from(button_atlas_handle));
    if let Some(ref mut atlas) = image_node.texture_atlas {
        atlas.index = index;
    }
    image_node
}
