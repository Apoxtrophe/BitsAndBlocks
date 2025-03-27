use crate::prelude::*;

pub fn create_identifier_timer () -> FadeTimer{
    // Spawn the fading text timer resource
    let timer = FadeTimer {
        timer: Timer::from_seconds(FADE_TIME, TimerMode::Once),
    };
    timer
}

pub fn spawn_identifier(
    commands: &mut Commands,
) -> Entity {
    
    let identifier_node = (Node {
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
    
    let voxel_identifier = commands.spawn((
        Text::new("Voxel Identifier"),
        text_settings,
        TextColor(Color::BLACK),
        TextLayout::new_with_justify(JustifyText::Center),
        identifier_node,
    )).insert(VoxelIdentifierText).id();
    
    voxel_identifier
}

/// Updates the voxel identifier text with a fade effect.
pub fn update_identifier(
    mut query: Query<(&mut Text, &mut TextColor, &mut VoxelIdentifierText)>,
    player: Res<Player>,
    mut fade_timer: ResMut<FadeTimer>,
    time: Res<Time>,
) {
    fade_timer.timer.tick(time.delta());
    
    // Return early if no identifier is selected.
    let descriptor = match &player.selected_descriptor {
        Some(descriptor) => descriptor,
        None => return,
    };

    let alpha = fade_timer.timer.fraction_remaining() + 0.25; // Remove the 0.25 if the text should fade entirely. 
    let new_color = Color::linear_rgba(0.75, 0.75, 0.75, alpha);
    
    for (mut text, mut text_color, _) in query.iter_mut() {
        text.0 = descriptor.name.clone();
        text_color.0 = new_color;
    }
}