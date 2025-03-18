use crate::prelude::*;

/// Spawns the cursor node at the center.
pub fn spawn_cursor_node(
    commands: &mut Commands,
    image: Handle<Image>,
) -> Entity {

    let image_node = ImageNode::new(image);
    let cursor_node = (Node {
        width: Val::VMin(2.0),
        height: Val::VMin(2.0),
        position_type: PositionType::Absolute,
        justify_self: JustifySelf::Center,
        align_self: AlignSelf::Center,
        ..default()
    },
    
    image_node,
    );

    commands.spawn(cursor_node).id()
}