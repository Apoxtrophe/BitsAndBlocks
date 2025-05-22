use crate::prelude::*;

pub fn setup_ui(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    image_handles: Res<GameTextures>,
) {
    // === Create Texture Atlases ===
    // Voxel texture atlas
    let voxel_texture_handle = image_handles.voxel_textures.clone();
    let voxel_atlas = TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        1,
        NUM_VOXELS as u32,
        None,
        None,
    );
    let voxel_atlas_handle = texture_atlases.add(voxel_atlas);

    // Button texture atlas
    let button_texture = image_handles.menu_button_texture.clone();
    let button_atlas = TextureAtlasLayout::from_grid(UVec2::new(144, 32), 1, 16, None, None);
    let button_atlas_handle = texture_atlases.add(button_atlas);
    
    // Cursor texture atlas
    let cursor_texture = image_handles.cursor_atlas.clone();
    let cursor_atlas = TextureAtlasLayout::from_grid(UVec2 { x: 16, y: 16 }, 2, 1, None, None);
    let cursor_atlas_handle = texture_atlases.add(cursor_atlas);

    // === Create Main UI Node ===
    let main_node = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(30.0)),
                column_gap: Val::Px(30.0),
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::Center,
                ..default()
            },
            GameEntity,
        ))
        .id();

    // === Spawn and Attach Child UI Components ===
    let children = vec![
        spawn_cursor_node(&mut commands,cursor_texture.clone(), cursor_atlas_handle),
        spawn_exit_menu(&mut commands, button_texture.clone(), button_atlas_handle.clone()),
        spawn_hotbar(&mut commands, &voxel_texture_handle, &voxel_atlas_handle),
        spawn_inventory(&mut commands, &voxel_texture_handle, &voxel_atlas_handle),
        spawn_identifier(&mut commands),
        spawn_debug_text(&mut commands),
    ];

    for child in children {
        commands.entity(child).set_parent(main_node);
    }
}

pub fn despawn_all(
    mut commands: Commands,
    entities: Query<Entity, With<GameEntity>>,
    camera_query: Query<Entity, With<PlayerCamera>>,
) {
    // Despawn all camera entities first
    for camera_entity in camera_query.iter() {
        commands.entity(camera_entity).despawn_recursive();
    }
    // Despawn all game entities
    for entity in entities.iter() {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn_recursive();
        }
    }
    println!("!!! GAME ENTITIES DESPAWNED");
}