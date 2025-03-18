use crate::prelude::*;



/// Defines the style for hotbar slots.
pub struct HotbarSlotStyle {
    pub size: Vec2,
    pub offset: Vec2,
    pub spread: f32,
    pub blur: f32,
    pub border_radius: BorderRadius,
}

/// Spawns an individual hotbar slot and its child image node.
pub fn spawn_hotbar_slot(
    parent: &mut ChildBuilder,
    index: usize,
    style: &HotbarSlotStyle,
    texture_handle: &Handle<Image>,
    texture_atlas_handle: &Handle<TextureAtlasLayout>,
) {
    let shadow_box = box_shadow_node_bundle(
        style.size, 
        style.offset, 
        style.spread, 
        style.blur, 
        style.border_radius
    );
    
    let image_node = (Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    HotbarSlot { index },
    ImageNode::from_atlas_image(
        texture_handle.clone(),
        TextureAtlas::from(texture_atlas_handle.clone()),
    ),
    );
    
    parent
        .spawn(shadow_box)
        .insert(HotbarSlot { index })
        .with_children(|child| {
            child.spawn(image_node);
        });
}


pub fn update_hotbar(
    player: ResMut<Player>,
    mut image_query: Query<(&HotbarSlot, &mut ImageNode)>,
    mut border_query: Query<(&HotbarSlot, &mut BorderColor)>,
    voxel_map: Res<VoxelMap>,
) {
    // Update border colors based on the player's selected slot.
    for (slot, mut border_color) in border_query.iter_mut() {
        *border_color = if slot.index == player.hotbar_selector {
            Color::WHITE.into() // Highlighted
        } else {
            Color::BLACK.into()
        };
    }

    // Update image atlas indices based on the hotbar selections.
    for (slot, mut image_node) in image_query.iter_mut() {
        let (_, sub_index) = player.hotbar_ids[slot.index];
        if let Some(atlas) = &mut image_node.texture_atlas {
            //atlas.index = texture_row((slot.index,sub_index));
            let id = (slot.index, sub_index);
            atlas.index = voxel_map.asset_map[&id].texture_row;
        }
    }
}