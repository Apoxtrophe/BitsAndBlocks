use crate::prelude::*;

pub fn setup_ui(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    image_handles: Res<GameTextures>,
) {
    commands.insert_resource(WhichUIShown {
        ui: WhichGameUI::Default,
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
        spawn_identifier(parent);
    });
}

pub fn update_game_window_visibility(

    mut query: Query<(&GameUIType, &mut Visibility)>,
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