use crate::prelude::*;

pub fn setup_ui(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    image_handles: Res<GameTextures>,
) {

    // ###
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

    // Prepare the cursor texture
    let cursor_texture_handle = image_handles.cursor_texture.clone();
    
    // Prepare button's texture atlas
    let buttons_texture = image_handles.menu_button_texture.clone();
    let button_atlas = TextureAtlasLayout::from_grid(UVec2::new(144, 32), 1, 16, None, None);
    let button_atlas_handle = texture_atlases.add(button_atlas);
    // ###
    
    // Create the main UI node.
    let main_node = commands.spawn((Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        padding: UiRect::all(Val::Px(30.0)),
        column_gap: Val::Px(30.0),
        flex_wrap: FlexWrap::Wrap,
        justify_content: JustifyContent::Center,
        ..default()
    },
    GameEntity)).id();
    
    // Spawn the cursor node as a child of main
    let cursor = spawn_cursor_node(&mut commands, cursor_texture_handle);
    commands.entity(cursor).set_parent(main_node);
    
    // Spawn the exit menu as a child of main
    let exit_menu = spawn_exit_menu(&mut commands, buttons_texture.clone(), button_atlas_handle.clone());
    commands.entity(exit_menu).set_parent(main_node);

    // Spawn the hotbar
    let hotbar = spawn_hotbar(&mut commands, &texture_handle, &texture_atlas_handle);
    commands.entity(hotbar).set_parent(main_node);
    
    // Spawn the inventory
    let inventory = spawn_inventory(&mut commands, &texture_handle, &texture_atlas_handle);
    commands.entity(inventory).set_parent(main_node);

    // Spawn the voxel_identifier above the hotbar
    let voxel_identifier = spawn_identifier(&mut commands);
    commands.entity(voxel_identifier).set_parent(main_node);
    
    // Spawn the debug_text
    let debug_text = spawn_debug_text(&mut commands);
    commands.entity(debug_text).set_parent(main_node);
}



pub fn update_game_window_visibility(

    mut query: Query<(&GameUI, &mut Visibility)>,
    current_screen: Res<GameUI>,
) {
    println!("{:?}", current_screen);
    //println!("current_ui: {:?}", current_screen.ui);
    for (ui, mut visibility) in query.iter_mut() {
        if *ui == *current_screen {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
        // Special case for allowing inventory and hotbar to be shown simultaneously
        if *ui == GameUI::Default && *current_screen == GameUI::Inventory {
            *visibility = Visibility::Visible;
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