use crate::prelude::*;

pub fn spawn_inventory(
    commands: &mut Commands,
    texture_handle: &Handle<Image>,
    texture_atlas_handle: &Handle<TextureAtlasLayout>,
) -> Entity {  
    let inventory_node = commands.spawn((Node {
        width: Val::Px(400.0),
        height: Val::Px(400.0),
        margin: UiRect::all(Val::Auto),
        flex_wrap: FlexWrap::Wrap,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        position_type: PositionType::Absolute,
        ..Default::default()
    },
    Visibility::Hidden,
    GameUI::Inventory,
    )).id();
    
    let button_node = (Node {
        width: Val::Percent(25.0),
        height: Val::Percent(25.0),
        margin: UiRect::all(Val::Auto),
        ..Default::default()
    }, 
    Visibility::Inherited,
    BackgroundColor(Color::WHITE),
    );

    let image_node = (Node {
        left: Val::Percent(5.0),
        top: Val::Percent(5.0),
        width: Val::Percent(90.0),
        height: Val::Percent(90.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        position_type: PositionType::Absolute,
        ..Default::default()
    },
    ImageNode::from_atlas_image(texture_handle.clone(), TextureAtlas::from(texture_atlas_handle.clone())),
    );
    
    commands.entity(inventory_node).with_children(|inventory_parent| {
        for i in 0..INVENTORY_SIZE {
            inventory_parent.spawn((Button, button_node.clone()))
                .insert(MenuAction::InventorySlot(i))
            .with_children(|child| {
                child.spawn(image_node.clone())
                    .insert(MenuAction::InventorySlot(i));
            });
        }
    });
    inventory_node
}


/// Updates inventory UI elements based on interactions and selected inventory slots.
pub fn update_inventory_ui(
    mut image_query: Query<(&MenuAction, &mut ImageNode)>,
    player: Res<Player>,
    voxel_map: Res<VoxelMap>,
) {   
    // Update inventory images.
    for (menu_action, mut image_node) in image_query.iter_mut() {
        match menu_action {
            MenuAction::InventorySlot(slot_id) => {
                let set = player.hotbar_selector;
                let subset = if *slot_id >= SUBSET_SIZES[set] { 0 } else { *slot_id };
                if let Some(atlas) = &mut image_node.texture_atlas {
                    let id = (set, subset);
                    atlas.index = voxel_map.asset_map[&id].texture_row;
                }
            }
            _ => {}
        }
    }
}




