use crate::prelude::*;

/// Spawns the debug text node.
pub fn spawn_debug_text(commands: &mut Commands) -> Entity {
    let text_node = (
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(10.0),
            left: Val::Percent(5.0),
            ..default()
        },
        DebugText,
        GameEntity,
    );
    
    let text_settings = TextFont {
        font_size: 18.0,
        ..default()
    };
    
    let debug_text = commands.spawn((
        Text::new("Initializing debug info..."),
        text_settings,
        TextColor(Color::BLACK),
        TextLayout::new_with_justify(JustifyText::Left),
        text_node,
    )).insert(GameUI::Debug).id();
    
    debug_text
}

/// Updates the debug text with runtime info.
pub fn update_debug_text(
    mut text_query: Query<&mut Text, With<DebugText>>,
    entity_query: Query<Entity>,
    player: Res<Player>,
    time: Res<Time>,
) {
    // Count entities for debugging purposes.
    let entity_count = entity_query.iter().count();
    
    // Compute FPS while avoiding division by zero.
    let delta = time.delta_secs().max(0.0001);
    let fps = (1.0 / delta).round();
    
    let debug_text = format!(
        "\
FPS: {:.1}
Delta Time: {:.3}s
Elapsed Time: {:.1}s

Camera Pos: {:.1}
Camera Direction: {:.1}

Ray Hit Pos: {:.1}
Hit Voxel: {:?}

Selected Voxel: {:?}
Selected Definition: {:?}
Voxel ID: {:?}

Hotbar: {:?}
Entity Count: {}",
        fps,
        delta,
        time.elapsed_secs().round(),
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
