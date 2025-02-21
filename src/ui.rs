

use bevy::{input::gamepad::ButtonSettings, prelude::*};

use crate::{config::{HOTBAR_BORDER_COLOR, NUM_VOXELS, SUBSET_SIZES, TEXTURE_PATH}, player::PlayerData, DebugText};

#[derive(Component)]
pub struct HotbarSlot {
    pub index: usize,
}

#[derive(Component)]
pub struct InventorySlot {
    pub index: usize,
}

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Spawn the debug text and cursor node.
    spawn_debug_text(&mut commands);
    spawn_cursor_node(&mut commands);

    // Load texture and create a texture atlas.
    let texture_handle: Handle<Image> = asset_server.load(TEXTURE_PATH);
    let texture_atlas = TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        1,
        NUM_VOXELS as u32,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Spawn the main UI node.
    let main_node = commands.spawn(main_ui_node()).id();

    // Define a common style for hotbar slots.
    let hotbar_style = HotbarSlotStyle {
        size: Vec2::splat(80.0),
        offset: Vec2::ZERO,
        spread: 4.0,
        blur: 20.0,
        border_radius: BorderRadius::all(Val::Percent(0.1)),
    };

    // Spawn 9 hotbar slots as children of the main node.
    commands.entity(main_node).with_children(|parent| {
        for i in 0..9 {
            spawn_hotbar_slot(
                parent,
                i,
                &hotbar_style,
                &texture_handle,
                &texture_atlas_handle,
            );
        }
    });
    
    commands.entity(main_node).with_children(|parent| {
        spawn_grid_menu(parent, &texture_handle, &texture_atlas_handle);
    });
}

#[derive(Component)]
pub struct GridMenu;


pub fn spawn_grid_menu(
    parent: &mut ChildBuilder,
    texture_handle: &Handle<Image>,
    texture_atlas_handle: &Handle<TextureAtlasLayout>,
) {
    parent.spawn((
    Node {
        width: Val::Px(320.0),
        height: Val::Px(320.0),
        margin: UiRect::all(Val::Auto),
        flex_wrap: FlexWrap::Wrap,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        position_type: PositionType::Absolute,
        ..Default::default()
    },
    Visibility::Visible,
    )).insert(GridMenu)
    .with_children(|grid_parent| {
        for i in 0..16 {
            grid_parent.spawn((Button, Node {
                width: Val::Percent(25.0),
                height: Val::Percent(25.0),
                margin: UiRect::all(Val::Auto),
                ..Default::default()
            },
            Visibility::Inherited,
            BackgroundColor(Color::WHITE))).insert(InventorySlot { index: i }).with_children(|child| {
                    child.spawn(Node {
                        left: Val::Percent(5.0),
                        top: Val::Percent(5.0),
                        width: Val::Percent(90.0),
                        height: Val::Percent(90.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    }).insert(ImageNode::from_atlas_image(texture_handle.clone(), TextureAtlas::from(texture_atlas_handle.clone())))
                    .insert(InventorySlot { index: i });
                });
                
        }
    });
}

//
/// Spawns the debug text node.
fn spawn_debug_text(commands: &mut Commands) {
    commands.spawn((
        Text::new("hello\nbevy!"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::BLACK),
        TextLayout::new_with_justify(JustifyText::Left),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(60.0),
            right: Val::Percent(5.0),
            ..default()
        },
        DebugText,
    ));
}

/// Spawns the cursor node at the center.
fn spawn_cursor_node(commands: &mut Commands) {
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
}

/// Returns the configuration for the main UI node.
fn main_ui_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        padding: UiRect::all(Val::Px(30.0)),
        column_gap: Val::Px(30.0),
        flex_wrap: FlexWrap::Wrap,
        justify_content: JustifyContent::Center,
        ..default()
    }
}

/// Defines the style for hotbar slots.
struct HotbarSlotStyle {
    size: Vec2,
    offset: Vec2,
    spread: f32,
    blur: f32,
    border_radius: BorderRadius,
}

/// Spawns an individual hotbar slot and its child image node.
fn spawn_hotbar_slot(
    parent: &mut ChildBuilder,
    index: usize,
    style: &HotbarSlotStyle,
    texture_handle: &Handle<Image>,
    texture_atlas_handle: &Handle<TextureAtlasLayout>,
) {
    parent
        .spawn(
            box_shadow_node_bundle(
                style.size,
                style.offset,
                style.spread,
                style.blur,
                style.border_radius,
            ),
        )
        .insert(HotbarSlot { index })
        .with_children(|child| {
            child.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                // Duplicate HotbarSlot for the child if needed.
                HotbarSlot { index },
                ImageNode::from_atlas_image(
                    texture_handle.clone(),
                    TextureAtlas::from(texture_atlas_handle.clone()),
                ),
            ));
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

pub fn update_inventory_ui(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &InventorySlot,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut query: Query<&mut Visibility, With<GridMenu>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut image_query: Query<(&InventorySlot, &mut ImageNode)>,
    mut player: ResMut<PlayerData>,
) {
    const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
    const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
    
    for (interaction, mut color, mut border_color, inventory_slot) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = PRESSED_BUTTON.into();
                let selector = player.selector.clone();
                let index = (inventory_slot.index).clamp(0, SUBSET_SIZES[selector] - 1);
                println!("{} :: {}", selector, index);
                player.hotbar_ids[selector].1 = index;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
    
    if keyboard_input.pressed(KeyCode::Tab) {
        for mut visibility in query.iter_mut() {
            *visibility = Visibility::Visible;
        }
    } else {
        for mut visibility in query.iter_mut() {
            *visibility = Visibility::Hidden;
        }
    }
    
    let offsets: Vec<_> = SUBSET_SIZES
        .iter()
        .scan(0, |state, &size| {
            let offset = *state;
            *state += size;
            Some(offset)
        })
        .collect();
    
    for (slot, mut image_node) in image_query.iter_mut() {
        let set = player.selector;
        let mut subset = (slot.index);
        
        if subset >= SUBSET_SIZES[set] {
            subset = 0;
        }
        if let Some(atlas) = &mut image_node.texture_atlas {
            atlas.index = subset + offsets[set];
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