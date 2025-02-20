use bevy::prelude::*;

use crate::{config::{HOTBAR_BORDER_COLOR, NUM_VOXELS, SUBSET_SIZES, TEXTURE_PATH}, player::PlayerData, DebugText};

#[derive(Component)]
pub struct HotbarSlot {
    pub index: usize,
}

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Spawn debug text.
    commands.spawn((
        // Accepts any type convertible into a `String`.
        Text::new("hello\nbevy!"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::BLACK),
        // Set text justification.
        TextLayout::new_with_justify(JustifyText::Left),
        // Style the text node.
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(60.0),
            right: Val::Percent(5.0),
            ..default()
        },
        DebugText,
    ));

    // Spawn cursor node at the center.
    commands.spawn((
        Node {
            width: Val::Percent(0.5),
            height: Val::Percent(1.0),
            left: Val::Percent(49.75),
            top: Val::Percent(49.5),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::BLACK),
    ));

    // Load texture and create a texture atlas layout.
    let texture_handle: Handle<Image> = asset_server.load(TEXTURE_PATH);
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(16), 1, NUM_VOXELS as u32, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Spawn the main UI node.
    let mut main_node = commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        padding: UiRect::all(Val::Px(30.0)),
        column_gap: Val::Px(30.0),
        flex_wrap: FlexWrap::Wrap,
        justify_content: JustifyContent::Center,
        ..default()
    });

    // Define hotbar slot styling.
    let hotbar_slot_style = (
        Vec2::splat(70.0),           // size
        Vec2::ZERO,                  // offset
        4.0,                         // spread
        20.0,                        // blur
        BorderRadius::all(Val::Percent(0.1)),
    );

    // Create 9 hotbar slots as children of the main node.
    main_node.with_children(|parent| {
        for i in 0..9 {
            parent
                .spawn(box_shadow_node_bundle(
                    hotbar_slot_style.0,
                    hotbar_slot_style.1,
                    hotbar_slot_style.2,
                    hotbar_slot_style.3,
                    hotbar_slot_style.4,
                ))
                .insert(HotbarSlot { index: i })
                .with_children(|child| {
                    child
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .insert(HotbarSlot { index: i })
                        .insert(ImageNode::from_atlas_image(
                            texture_handle.clone(),
                            TextureAtlas::from(texture_atlas_handle.clone()),
                        ));
                });
        }
    });
}

pub fn update_hotbar(
    player: ResMut<PlayerData>,
    mut image_query: Query<(&HotbarSlot, &mut ImageNode)>,
    mut border_query: Query<(&HotbarSlot, &mut BorderColor)>,
) {
    // Compute cumulative offsets for each hotbar subset.
    let offsets: Vec<_> = SUBSET_SIZES
        .iter()
        .scan(0, |state, &size| {
            let offset = *state;
            *state += size;
            Some(offset)
        })
        .collect();

    // Update border colors based on the player's selected slot.
    for (slot, mut border_color) in border_query.iter_mut() {
        *border_color = if slot.index == player.selector {
            Color::WHITE.into() // Highlighted
        } else {
            Color::BLACK.into()
        };
    }

    // Update image atlas indices based on the hotbar selections.
    for (slot, mut image_node) in image_query.iter_mut() {
        let (_, sub_index) = player.hotbar_ids[slot.index];
        if let Some(atlas) = &mut image_node.texture_atlas {
            atlas.index = offsets[slot.index] + sub_index;
        }
    }
}

fn box_shadow_node_bundle(
    size: Vec2,
    offset: Vec2,
    spread: f32,
    blur: f32,
    border_radius: BorderRadius,
) -> impl Bundle {
    (
        Node {
            top: Val::Percent(90.0),
            width: Val::Px(size.x),
            height: Val::Px(size.y),
            border: UiRect::all(Val::Px(6.0)),
            ..default()
        },
        BorderColor(HOTBAR_BORDER_COLOR.into()),
        border_radius,
        BackgroundColor(Color::linear_rgba(0.2, 0.2, 0.2, 0.1)),
        BoxShadow {
            color: Color::BLACK.with_alpha(0.8),
            x_offset: Val::Percent(offset.x),
            y_offset: Val::Percent(offset.y),
            spread_radius: Val::Percent(spread),
            blur_radius: Val::Px(blur),
        },
    )
}

pub fn update_text(
    mut text_query: Query<&mut Text, With<DebugText>>,
    entity_query: Query<Entity>,
    player: Res<PlayerData>,
) {
    let entity_count = entity_query.iter().count();

    // Create the debug text string.
    let debug_text = format!(
        "\
Camera Pos: {:.1}
Camera Direction: {:.1}
Ray Hit: {:.1}
Selected Block: {:.1}
Selected Adj.: {:.1}
Voxel ID: {:?}
Hotbar: {:?}
Entity Count: {}",
        player.camera_pos,
        player.camera_dir,
        player.ray_hit_pos,
        player.selected,
        player.selected_adjacent,
        player.selector,
        player.hotbar_ids,
        entity_count,
    );

    // Update all debug text entities.
    for mut text in text_query.iter_mut() {
        text.0 = debug_text.clone();
    }
}
