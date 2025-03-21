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
    InventoryGrid,
    GameUIType { ui: WhichGameUI::Inventory },
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
                .insert(InventorySlot { index: i })
            .with_children(|child| {
                child.spawn(image_node.clone())
                    .insert(InventorySlot { index: i });
            });
        }
    });
    
    inventory_node
}

pub fn update_inventory_ui(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &InventorySlot,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut image_query: Query<(&InventorySlot, &mut ImageNode)>,
    mut player: ResMut<Player>,
    voxel_map: Res<VoxelMap>,
) {   
    for (interaction, mut color, inventory_slot) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.15, 0.90, 0.15).into();
                let selector = player.hotbar_selector.clone();
                let index = (inventory_slot.index).clamp(0, SUBSET_SIZES[selector] - 1);
                player.hotbar_ids[selector].1 = index;
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.5, 0.5, 0.5).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.15, 0.15, 0.15).into();
            }
        }
    }

    for (slot, mut image_node) in image_query.iter_mut() {
        let set = player.hotbar_selector;
        let mut subset = slot.index;
        
        if subset >= SUBSET_SIZES[set] {
            subset = 0;
        }
        if let Some(atlas) = &mut image_node.texture_atlas {
            let id = (set, subset);
            atlas.index = voxel_map.asset_map[&id].texture_row;
        }
    }
}