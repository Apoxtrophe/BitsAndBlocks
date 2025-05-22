use crate::prelude::*;

#[derive(Component)]
pub struct Cursor;

/// Spawns the cursor node at the center.
pub fn spawn_cursor_node(
    commands: &mut Commands,
    image: Handle<Image>,
    handle: Handle<TextureAtlasLayout>,
) -> Entity {

    
    let mut image_node = ImageNode::from_atlas_image(image, TextureAtlas::from(handle));
    
    if let Some(atlas) = &mut image_node.texture_atlas {
        atlas.index = 0;
    }
    
    let mut mouse_cursor = commands.spawn((
        Node {
            width: Val::VMin(4.0),
            height: Val::VMin(4.0),
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            ..default()
        },
        image_node,
        Cursor,
    )).id(); 
    
    mouse_cursor
}

pub fn update_cursor(
    mut cursor_query: Query<(&mut ImageNode, &Cursor),>,
    player: Res<Player>,
) {

    
    for mut image_node in &mut cursor_query {
        if let Some(atlas) = &mut image_node.0.texture_atlas {
            
            if let Some(voxel) = player.hit_voxel {
                if voxel.kind == VoxelType::Component(ComponentVariants::Button) || voxel.kind == VoxelType::Component(ComponentVariants::Switch) {
                    atlas.index = 1;
                } else { 
                    atlas.index = 0;
                }
            } else { 
                atlas.index = 0;
            }
            

        }
    }
}