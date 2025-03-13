use bevy::{prelude::*, window::CursorGrabMode};

use crate::{prelude::*, GameState};

#[derive(Component)]
pub struct DebugText;

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

#[derive(Component)]
pub struct VoxelIdentifierText;

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub struct UIType {
    pub ui: WhichUI,
}

#[derive(Resource, PartialEq, Eq, Clone, Copy)]
pub struct WhichUIShown {
    pub ui: WhichUI,
}

// Local resource for InGame that keeps track of which toggleable ui is shown. 
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WhichUI {
    Default,
    Inventory,
    HotbarHidden,
    ExitMenu,
}

#[derive(Component)]
pub struct GameEntity; // Entities that are removed after leaving the game state.

/// Defines the style for hotbar slots.
struct HotbarSlotStyle {
    size: Vec2,
    offset: Vec2,
    spread: f32,
    blur: f32,
    border_radius: BorderRadius,
}

pub fn create_definition_timer () -> FadeTimer{
    // Spawn the fading text timer resource
    let timer = FadeTimer {
        timer: Timer::from_seconds(FADE_TIME, TimerMode::Once),
    };
    
    timer
}

pub fn setup_ui(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    image_handles: Res<GameTextures>,
) {
    commands.insert_resource(WhichUIShown {
        ui: WhichUI::Default,
    });
    
    // Spawn the debug text and cursor node.

    spawn_debug_text(&mut commands);


    // Load texture and create a texture atlas.
    let texture_handle: Handle<Image> = image_handles.voxel_textures.clone();

    let texture_atlas = TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        1,
        NUM_VOXELS as u32,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let main_node = (Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        padding: UiRect::all(Val::Px(30.0)),
        column_gap: Val::Px(30.0),
        flex_wrap: FlexWrap::Wrap,
        justify_content: JustifyContent::Center,
        ..default()
    },
    GameEntity);
    
    // Spawn the main UI node.
    let main_node = commands.spawn(main_node).id();
    
    let cursor_texture_handle = image_handles.cursor_texture.clone();
    let cursor = spawn_cursor_node(&mut commands, cursor_texture_handle);
    commands.entity(cursor).set_parent(main_node);
    
    let exit_menu = spawn_exit_menu(&mut commands);
    commands.entity(exit_menu).set_parent(main_node);
    
    let sub_exit_menu =  spawn_sub_node(&mut commands, 30.0, 70.0, 15.0);
    commands.entity(sub_exit_menu).set_parent(exit_menu);
    
    // Prepare button texture atlas
    let buttons_texture = image_handles.menu_button_texture.clone();
    let button_atlas = TextureAtlasLayout::from_grid(UVec2::new(144, 32), 1, 16, None, None);
    let button_atlas_handle = texture_atlases.add(button_atlas);
    
    for i in 0..4 {
        spawn_button (
            &mut commands,
            sub_exit_menu,
            buttons_texture.clone(),
            button_atlas_handle.clone(),
            i + 8,
            24.0,
        );
    }

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
        spawn_inventory(parent, &texture_handle, &texture_atlas_handle);
    });
    
    // Spawn Voxel Identifier text above hotbar
    commands.entity(main_node).with_children(|parent| {
        spawn_voxel_identifier(parent);
    });
}

fn spawn_exit_menu (
    commands: &mut Commands,
) -> Entity {
    spawn_ui_node(
        commands,
        Node {
            width: Val::Percent(80.0),
            height: Val::Percent(80.0),
            flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        (BackgroundColor(Color::linear_rgba(0.1, 0.1, 0.1, 0.8)), UIType { ui: WhichUI::ExitMenu }, Visibility::Hidden),
    )
}

pub fn exit_menu_interaction(
    mut query: Query<(&Interaction, &mut BackgroundColor, &ButtonNumber), (Changed<Interaction>, With<Button>)>,
    mut which_ui: ResMut<WhichUIShown>,
    mut event_writer: EventWriter<GameEvent>,
    mut app_state: ResMut<NextState<GameState>>,
) {
    
    for (interaction, mut bg_color, button_number) in query.iter_mut() {
        match *interaction {
            
            Interaction::Pressed => {                
                match button_number.index {
                    8 => {
                        println!("Back To Game");
                        which_ui.ui = WhichUI::Default;
                        event_writer.send(GameEvent::UpdateCursor {
                            mode: CursorGrabMode::Locked,
                            show_cursor: false,
                            enable_input: true,
                        });
                    }
                    9 => {
                        println!("Main Menu");

                        
                        app_state.set(GameState::Loading);
                    }
                    10 => {
                        println!("Save & Quit");
                    }
                    11 => {
                        println!("Placeholder");
                    }
                    _ => {}
                }

                *bg_color = Color::linear_rgba(0.0, 1.0, 0.0, 1.0).into();
            }
            Interaction::Hovered => {
                *bg_color = Color::linear_rgba(1.0, 1.0, 1.0, 1.0).into();
            }
            Interaction::None => {
                *bg_color = Color::linear_rgba(0.0, 0.0, 0.0, 0.0).into();
            }
        }
    }
    
}
pub fn spawn_voxel_identifier(
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
            color.0 = Color::linear_rgba(1.0, 1.0, 1.0, alpha);
        }
    } else {
        return;
    }
}

/// Spawns the debug text node.
fn spawn_debug_text(commands: &mut Commands) {
    let text_node = (Node {
        position_type: PositionType::Absolute,
        bottom: Val::Percent(60.0),
        right: Val::Percent(5.0),
        ..default()
    },
    DebugText,
    GameEntity);
    
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

pub fn update_debug_text(
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

/// Spawns the cursor node at the center.
fn spawn_cursor_node(
    commands: &mut Commands,
    image: Handle<Image>,
) -> Entity {

    let image_node = ImageNode::new(image);
    let cursor_node = (Node {
        width: Val::VMin(2.0),
        height: Val::VMin(2.0),
        position_type: PositionType::Absolute,
        justify_self: JustifySelf::Center,
        align_self: AlignSelf::Center,
        ..default()
    },
    
    image_node,
    );

    commands.spawn(cursor_node).id()
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

pub fn spawn_inventory(
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
    Visibility::Hidden,
    GridMenu,
    UIType { ui: WhichUI::Inventory },
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

pub fn update_game_window_visibility(

    mut query: Query<(&UIType, &mut Visibility)>,
    current_screen: Res<WhichUIShown>,
) {
    for (ui, mut visibility) in query.iter_mut() {
        if ui.ui == current_screen.ui {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn despawn_all(
    mut commands: Commands,
    entities: Query<Entity, With<GameEntity>>,
    camera_query: Query<Entity, With<PlayerCamera>>,
) {
    
    
    for camera_entity in camera_query.iter() {
        commands.entity(camera_entity).despawn_recursive();
    }
    for entity in entities.iter() {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn_recursive();
        }
    }
    
    
    
    
    

    println!("Entities Despawned");
}