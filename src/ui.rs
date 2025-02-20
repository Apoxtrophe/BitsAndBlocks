use bevy::{color::palettes::css::{DEEP_SKY_BLUE, LIGHT_SKY_BLUE}, prelude::*, render::render_resource::encase::private::RuntimeSizedArray};

use crate::{config::{HOTBAR_BORDER_COLOR, NUM_VOXELS, SUBSET_SIZES, TEXTURE_PATH}, player::PlayerData, voxel::VoxelAssets, DebugText};

#[derive(Component)]
pub struct HotbarSlot {
    pub index: usize,
}

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Spawn debug text
    commands.spawn((
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new("hello\nbevy!"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::BLACK),
        // Set the justification of the Text
        TextLayout::new_with_justify(JustifyText::Left),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(60.0),
            right: Val::Percent(5.0),
            ..default()
        },
        DebugText,
    ));
    
    // Spawn cursor
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

    let texture_handle: Handle<Image> = asset_server.load(TEXTURE_PATH);
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(16), 1, NUM_VOXELS as u32, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    let mut main_node = commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        padding: UiRect::all(Val::Px(30.)),
        column_gap: Val::Px(30.),
        flex_wrap: FlexWrap::Wrap,
        justify_content: JustifyContent::Center,
        ..default()
    });
    
    let hot_box = [(
        Vec2::splat(70.),
        Vec2::ZERO,
        4.,
        20.,
        BorderRadius::all(Val::Percent(0.1)),
        ),
    ];
    
    let mut hotbar = [hot_box;9];
    
    main_node.with_children(|commands| {
        for i in 0..9{
            for (size, offset, spread, blur, border_radius) in hotbar[i] {
                commands.spawn(box_shadow_node_bundle(
                    size,
                    offset,
                    spread,
                    blur,
                    border_radius,
                )).insert(HotbarSlot{index: i}).with_children(|child| {
                    child.spawn(Node {
                        width: Val::Percent(95.0),
                        height: Val::Percent(95.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },).insert(HotbarSlot{index: i}).insert(ImageNode::from_atlas_image(texture_handle.clone(), TextureAtlas::from(texture_atlas_handle.clone())));
                });
            }
        }  
    });
}

pub fn update_hotbar (
    player: ResMut<PlayerData>,
    mut query: Query<(&HotbarSlot, &mut ImageNode)>,
    mut query2: Query<(&HotbarSlot, &mut BorderColor)>,
    
) { 
    let mut count = 0; 
    let mut offsets = Vec::new();
    
    for i in 0..SUBSET_SIZES.len() {
        offsets.push(count);
        count += SUBSET_SIZES[i];
    }
    
    for (slot, mut color) in query2.iter_mut() {
        if slot.index == player.selector {
            // Highlight the selected slot (e.g., brighter color)
            *color = Color::WHITE.into();
        } else {
            *color = Color::BLACK.into();
        }
    }
    for (slot, mut image) in query.iter_mut() {

        let index = player.hotbar_ids[slot.index];
        
        if let Some(atlas) = &mut image.texture_atlas {
            atlas.index = offsets[slot.index] + index.1;
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
            border: UiRect::all(Val::Px(6.)),
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
    mut query: Query<&mut Text, With<DebugText>>,
    entity_query: Query<Entity>,
    player: Res<PlayerData>,
) {
    let entitiy_count = entity_query.iter().count();
    
    let debug_text = format!(
        "
        Camera Pos: {:.1}
        Camera Direction: {:.1}
        Ray Hit: {:.1}
        Selected Block: {:.1}
        Selected Adj.: {:.1}
        Voxel ID: {:?}
        Hotbar: {:?}
        Entity Count: {}
        ", 
        player.camera_pos, 
        player.camera_dir, 
        player.ray_hit_pos, 
        player.selected, 
        player.selected_adjacent,
        player.selector,
        player.hotbar_ids,
        entitiy_count,
    );
    
    for mut text in &mut query {
        text.0 = debug_text.clone();
    }
}
