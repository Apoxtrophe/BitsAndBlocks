use bevy::prelude::*;
use crate::{config::{FADE_TIME, HOTBAR_BORDER_COLOR, NUM_VOXELS, SUBSET_SIZES, TEXTURE_PATH}, player::Player, voxel::VoxelMap, DebugText};


#[derive(Component)]
pub struct GridMenu;


#[derive(Component)]
pub struct HotbarSlot {
    pub index: usize,
}

#[derive(Component)]
pub struct InventorySlot {
    pub index: usize,
}

/// Defines the style for hotbar slots.
struct HotbarSlotStyle {
    size: Vec2,
    offset: Vec2,
    spread: f32,
    blur: f32,
    border_radius: BorderRadius,
}

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Spawn the fading text timer resource
    let timer = FadeTimer {
        timer: Timer::from_seconds(FADE_TIME, TimerMode::Once),
    };
    
    commands.insert_resource(timer);
    
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

    let main_node = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        padding: UiRect::all(Val::Px(30.0)),
        column_gap: Val::Px(30.0),
        flex_wrap: FlexWrap::Wrap,
        justify_content: JustifyContent::Center,
        ..default()
    };
    
    // Spawn the main UI node.
    let main_node = commands.spawn(main_node).id();

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
    
    // Spawn inventory grid
    commands.entity(main_node).with_children(|parent| {
        spawn_grid_menu(parent, &texture_handle, &texture_atlas_handle);
    });
    
    commands.entity(main_node).with_children(|parent| {
        setup_voxel_identifier(parent);
    });
}

#[derive(Component)]
pub struct VoxelIdentifierText;

#[derive(Resource)]
pub struct FadeTimer {
    pub timer: Timer,
}

pub fn setup_voxel_identifier(
    parent: &mut ChildBuilder,
) {

    
    let main_node = (Node {
        width: Val::Percent(50.0),
        height: Val::Percent(5.0),
        bottom: Val::Percent(15.0),
        position_type: PositionType::Absolute,
        ..default()
    },
    );
    
    let text_settings = TextFont {
        font_size: 32.0,
        ..default()
    };
    
    parent.spawn((
        Text::new("Voxel Identifier"),
        text_settings,
        TextColor(Color::BLACK),
        TextLayout::new_with_justify(JustifyText::Center),
        main_node,
        )).insert(VoxelIdentifierText);
}

pub fn update_voxel_identifier(
    mut query: Query<(&mut Text, &mut TextColor,  &mut VoxelIdentifierText)>,
    player: Res<Player>,
    mut fade_timer: ResMut<FadeTimer>,
    time: Res<Time>,
) {
    fade_timer.timer.tick(time.delta());
    
    if let Some(voxel_identifier) = player.selected_descriptor.clone() {
        for (mut text, mut color, _) in query.iter_mut() {
            text.0 = voxel_identifier.name.clone();
            let alpha = fade_timer.timer.fraction_remaining();
            println!("{}", alpha);
            color.0 = Color::linear_rgba(1.0, 1.0, 1.0, alpha);
        }
    } else {
        return;
    }
}

pub fn spawn_grid_menu(
    parent: &mut ChildBuilder,
    texture_handle: &Handle<Image>,
    texture_atlas_handle: &Handle<TextureAtlasLayout>,
) {  
    let grid_node = (Node {
        width: Val::Px(400.0),
        height: Val::Px(400.0),
        margin: UiRect::all(Val::Auto),
        flex_wrap: FlexWrap::Wrap,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        position_type: PositionType::Absolute,
        ..Default::default()
    },
    Visibility::Visible,
    GridMenu,
    );
    
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
    
    parent.spawn(grid_node)
    .with_children(|grid_parent| {
        for i in 0..16 {
            grid_parent.spawn((Button, button_node.clone()))
                .insert(InventorySlot { index: i })
            .with_children(|child| {
                child.spawn(image_node.clone())
                    .insert(InventorySlot { index: i });
            });
        }
    });
}

/// Spawns the debug text node.
fn spawn_debug_text(commands: &mut Commands) {
    let text_node = (Node {
        position_type: PositionType::Absolute,
        bottom: Val::Percent(60.0),
        right: Val::Percent(5.0),
        ..default()
    },
    DebugText);
    
    let text_settings = TextFont {
        font_size: 16.0,
        ..default()
    };
    
    commands.spawn((
        Text::new("hello\nbevy!"),
        text_settings,
        TextColor(Color::BLACK),
        TextLayout::new_with_justify(JustifyText::Left),
        text_node,
    ));
}

/// Spawns the cursor node at the center.
fn spawn_cursor_node(commands: &mut Commands) {
    let cursor_node = (Node {
        width: Val::Percent(1.0),
        height: Val::Percent(1.0),
        left: Val::Percent(49.75),
        top: Val::Percent(49.5),
        position_type: PositionType::Absolute,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    BackgroundColor(Color::BLACK),
    );

    commands.spawn(cursor_node);
}

/// Spawns an individual hotbar slot and its child image node.
fn spawn_hotbar_slot(
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
    mut image_query: Query<(&InventorySlot, &mut ImageNode)>,
    mut player: ResMut<Player>,
    voxel_map: Res<VoxelMap>,
) {
    const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
    const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
    
    for (interaction, mut color, mut border_color, inventory_slot) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = PRESSED_BUTTON.into();
                let selector = player.hotbar_selector.clone();
                let index = (inventory_slot.index).clamp(0, SUBSET_SIZES[selector] - 1);
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
    player: Res<Player>,
) {
    let entity_count = entity_query.iter().count();

    // Create the debug text string.
    let debug_text = format!(
        "\
Camera Pos: {:.1}
Camera Direction: {:.1}
Ray Hit: {:.1}
Hit Voxel: {:?}
Selected Voxel.: {:?}
Selected Definition: {:?}
Voxel ID: {:?}
Hotbar: {:?}
Entity Count: {}",
        player.camera_pos,
        player.camera_dir,
        player.ray_hit_pos,
        player.hit_voxel,
        player.selected_voxel,
        player.selected_descriptor,
        player.hotbar_selector,
        player.hotbar_ids,
        entity_count,
    );

    // Update all debug text entities.
    for mut text in text_query.iter_mut() {
        text.0 = debug_text.clone();
    }
}