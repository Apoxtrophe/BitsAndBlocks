use std::time::Duration;

use crate::prelude::*;

/// Spawns a UI node that displays the voxel identifier text.
pub fn spawn_identifier(commands: &mut Commands) -> Entity {
    // Container node for the identifier text.
    let identifier_node = Node {
        width: Val::Percent(50.0),
        height: Val::Percent(5.0),
        bottom: Val::Percent(15.0),
        position_type: PositionType::Absolute,
        ..default()
    };

    // Text style settings.
    let text_settings = TextFont {
        font_size: 32.0,
        ..default()
    };

    // Spawn the text entity and tag it as a voxel identifier.
    commands
        .spawn((
            identifier_node,
            Text::new("Voxel Identifier"),
            text_settings,
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(JustifyText::Center),
        ))
        .insert(VoxelIdentifierText)
        .id()
}


/// Updates the voxel identifier text with a fade effect.
pub fn update_identifier(
    mut query: Query<(&mut Text, &mut TextColor, &mut VoxelIdentifierText)>,
    player: Res<Player>,
    time: Res<Time>,
    mut previous_selected: Local<usize>,
    mut fade_timer: Local<Timer>,
) {
    fade_timer.tick(time.delta());

    // Exit early if no descriptor is selected.
    let descriptor = match &player.selected_descriptor {
        Some(descriptor) => descriptor,
        None => return,
    };

    // Calculate alpha with a baseline offset (remove 0.25 for full fade-out).
    let alpha = fade_timer.fraction_remaining() + 0.25;
    let new_color = Color::linear_rgba(0.85, 0.85, 0.85, alpha);

    // Update text and color for all identifier components.
    for (mut text, mut text_color, _) in query.iter_mut() {
        text.0 = descriptor.name.clone();
        text_color.0 = new_color;
    }

    // Reset the timer if the selected hotbar slot changes.
    if player.hotbar_selector != *previous_selected {
        fade_timer.reset();
        fade_timer.set_duration(Duration::from_secs(FADE_TIME as u64));
    }
    *previous_selected = player.hotbar_selector;
}