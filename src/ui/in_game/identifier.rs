use std::time::Duration;

use crate::prelude::*;

const IDENTIFIER_WIDTH_PERCENT: f32 = 50.0;
const IDENTIFIER_HEIGHT_PERCENT: f32 = 5.0;
const IDENTIFIER_BOTTOM_PERCENT: f32 = 15.0;
const IDENTIFIER_FONT_SIZE: f32 = 32.0;

/// Spawns a UI node that displays the voxel identifier text.
pub fn spawn_identifier(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Node {
                width: Val::Percent(IDENTIFIER_WIDTH_PERCENT),
                height: Val::Percent(IDENTIFIER_HEIGHT_PERCENT),
                bottom: Val::Percent(IDENTIFIER_BOTTOM_PERCENT),
                position_type: PositionType::Absolute,
                ..default()
            },
            Text::new("Voxel Identifier"),
            TextFont {
                font_size: IDENTIFIER_FONT_SIZE,
                ..default()
            },
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(JustifyText::Center),
            GameUI::Default,
            VoxelIdentifierText,
        ))
        .id()
    
}


/// Updates the voxel identifier text with a fade effect.
pub fn update_identifier(
    mut query: Query<(&mut Text, &mut TextColor), With<VoxelIdentifierText>>,
    player: Res<Player>,
    time: Res<Time>,
    mut previous_selected: Local<usize>,
    mut fade_timer: Local<Option<Timer>>,
) {
    let Some(descriptor) = &player.selected_descriptor else {
        return;
    };
    
    let timer = fade_timer.get_or_insert_with(|| new_fade_timer());
    timer.tick(time.delta());

    if player.hotbar_selector != *previous_selected {
        *timer = new_fade_timer();
    }
    *previous_selected = player.hotbar_selector;

    let alpha = timer.fraction_remaining() + 0.25;
        
    let new_color = Color::linear_rgba(0.85, 0.85, 0.85, alpha);

    // Update text and color for all identifier components.
    for (mut text, mut text_color) in query.iter_mut() {
        text.0 = descriptor.name.clone();
        text_color.0 = new_color;
    }
}

fn new_fade_timer() -> Timer {
    Timer::new(Duration::from_secs(FADE_TIME as u64), TimerMode::Once)
}