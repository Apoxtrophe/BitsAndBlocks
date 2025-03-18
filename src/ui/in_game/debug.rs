use crate::prelude::*;

/// Spawns the debug text node.
pub fn spawn_debug_text(commands: &mut Commands) {
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
