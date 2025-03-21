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

pub fn update_identifier(
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
