use crate::prelude::*;

/// Spawns the inventory UI and its slot buttons.
pub fn spawn_inventory(
    commands: &mut Commands,
    texture_handle: &Handle<Image>,
    texture_atlas_handle: &Handle<TextureAtlasLayout>,
) -> Entity {
    // Create the inventory container node.
    let inventory_node = commands
        .spawn((
            Node {
                width: Val::VMin(50.0),
                height: Val::VMin(50.0),
                margin: UiRect::all(Val::Auto),
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            BackgroundColor(Color::linear_rgba(0.1, 0.1, 0.1, 0.9)),
            Visibility::Hidden,
            GameUI::Inventory(0),
        ))
        .id();

    // Define a reusable button bundle for inventory slots.
    let button_bundle = (
        Node {
            width: Val::Percent(23.),
            height: Val::Percent(23.),
            margin: UiRect::all(Val::Auto),
            ..Default::default()
        },
        Visibility::Inherited,
        BackgroundColor(Color::WHITE),
    );

    // Define a reusable image bundle for the inventory slot icons.
    let image_bundle = (
        Node {
            left: Val::Percent(5.0),
            top: Val::Percent(5.0),
            width: Val::Percent(90.0),
            height: Val::Percent(90.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        ImageNode::from_atlas_image(
            texture_handle.clone(),
            TextureAtlas::from(texture_atlas_handle.clone()),
        ),
    );

    // Spawn each inventory slot button with its child image.
    commands.entity(inventory_node).with_children(|inventory_parent| {
        for i in 0..INVENTORY_SIZE {
            inventory_parent
                .spawn((Button, button_bundle.clone()))
                .insert(MenuAction::InventorySlot(i))
                .insert(GameUI::Inventory(i))
                .with_children(|child| {
                    child
                        .spawn(image_bundle.clone())
                        .insert(MenuAction::InventorySlot(i))
                        .insert(GameUI::Inventory(i));
                });
        }
    });
    inventory_node
}

/// Refreshes every inventory‑slot icon so it shows the sprite that corresponds
/// to the (group, sub‑index) pair under the new `VoxelType` enum scheme.
pub fn update_inventory(
    mut image_query: Query<(&MenuAction, &mut ImageNode)>,
    player:       Res<Player>,
    voxel_map:    Res<VoxelMap>,
) {
    for (menu_action, mut img_node) in image_query.iter_mut() {
        if let MenuAction::InventorySlot(slot_id) = menu_action {
            let group   = player.hotbar_selector;               // current hot‑bar group
            let subset  = if *slot_id >= SUBSET_SIZES[group] {   // clamp to valid range
                0
            } else {
                *slot_id
            };

            // Build the enum variant that this slot represents.
            if let Ok(kind) = VoxelType::try_from((group, subset)) {
                if let (Some(atlas), Some(asset)) =
                    (&mut img_node.texture_atlas, voxel_map.asset_map.get(&kind))
                {
                    atlas.index = asset.texture_row;             // set sprite
                }
            }
        }
    }
}