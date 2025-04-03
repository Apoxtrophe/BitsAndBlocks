use crate::prelude::*;

/// Defines the style for hotbar slots.
pub struct HotbarSlotStyle {
    pub vmin_size: f32,
    pub offset: Vec2,
    pub spread: f32,
    pub blur: f32,
    pub border_radius: BorderRadius,
}

/// Spawns the hotbar UI container and its slots.
pub fn spawn_hotbar(
    commands: &mut Commands, 
    texture_handle: &Handle<Image>,
    texture_atlas_handle: &Handle<TextureAtlasLayout>,
) -> Entity {
    // Create the hotbar container node.
    let hotbar_node = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(10.0),
            top: Val::Percent(90.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.0),
            ..default()
        },
    )).id();

    // Define a common style for hotbar slots.
    let hotbar_style = HotbarSlotStyle {
        vmin_size: 10.0,
        offset: Vec2::ZERO,
        spread: 4.0,
        blur: 20.0,
        border_radius: BorderRadius::all(Val::Percent(0.1)),
    };

    // Spawn and attach each hotbar slot.
    for i in 0..HOTBAR_SIZE {
        let slot = spawn_hotbar_slot(
            commands,
            i,
            &hotbar_style,
            texture_handle,
            texture_atlas_handle,
        );
        commands.entity(slot).set_parent(hotbar_node);
    }
    hotbar_node
}

/// Spawns an individual hotbar slot with its container and image node.
pub fn spawn_hotbar_slot(
    commands: &mut Commands,
    index: usize,
    style: &HotbarSlotStyle,
    texture_handle: &Handle<Image>,
    texture_atlas_handle: &Handle<TextureAtlasLayout>,
) -> Entity {
    // Create the slot container with shadow and border.
    let slot_entity = commands
        .spawn(hotslot_bundle(
            style.vmin_size, 
            style.offset, 
            style.spread, 
            style.blur, 
            style.border_radius,
        ))
        .id();

    // Spawn the image node that displays the slot's texture.
    let image_entity = commands
        .spawn((
            Node {
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
        ))
        .id();

    // Attach components to the slot container.
    commands.entity(slot_entity)
        .insert(HotbarSlot { index })
        .insert(Visibility::Visible)
        .insert(GameUI::Default);

    // Set the image node as a child of the slot container.
    commands.entity(image_entity).set_parent(slot_entity);

    slot_entity
}

/// Creates the bundle for a hotbar slot with its shadow and border.
pub fn hotslot_bundle(
    vmin_size: f32,
    offset: Vec2,
    spread: f32,
    blur: f32,
    border_radius: BorderRadius,
) -> impl Bundle {
    (
        Node {
            width: Val::VMin(vmin_size),
            height: Val::VMin(vmin_size),
            border: UiRect::all(Val::Px(6.0)),
            ..default()
        },
        BorderColor(HOTBAR_BORDER_COLOR.into()),
        border_radius,
        BackgroundColor(Color::linear_rgba(0.2, 0.2, 0.2, 0.1)),
        BoxShadow {
            color: Color::BLACK.with_alpha(1.0),
            x_offset: Val::Percent(offset.x),
            y_offset: Val::Percent(offset.y),
            spread_radius: Val::Percent(spread),
            blur_radius: Val::Px(blur),
        },
    )
}

/// Updates the hotbar visuals based on the player's current selection.
pub fn update_hotbar(
    player: Res<Player>,
    mut image_query: Query<(&HotbarSlot, &mut ImageNode)>,
    mut border_query: Query<(&HotbarSlot, &mut BorderColor)>,
    voxel_map: Res<VoxelMap>,
) {
    // Update border colors to highlight the selected slot.
    for (slot, mut border_color) in border_query.iter_mut() {
        *border_color = if slot.index == player.hotbar_selector {
            BORDER_SELECTED.into()
        } else {
            BORDER_UNSELECTED.into()
        };
    }

    // Update the image atlas indices for each hotbar slot.
    for (slot, mut image_node) in image_query.iter_mut() {
        let (_, sub_index) = player.hotbar_ids[slot.index];
        if let Some(atlas) = &mut image_node.texture_atlas {
            let id = (slot.index, sub_index);
            atlas.index = voxel_map.asset_map[&id].texture_row;
        }
    }
}