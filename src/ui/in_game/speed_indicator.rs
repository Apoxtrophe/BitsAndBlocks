use crate::prelude::*;

const SPEED_INDICATOR_TOP: f32 = 20.0; 
const SPEED_INDICATOR_LEFT: f32 = 40.0;

/// Spawns the speed indicator widget used to show the current simulation rate.
pub fn spawn_speed_indicator(
    commands: &mut Commands,
    speed_indicator_texture: Handle<Image>,
    speed_indicator_atlas: Handle<TextureAtlasLayout>,
) -> Entity {
    commands
        .spawn((
            Node {
                width: Val::VMin(20.0),
                height: Val::VMin(10.0),
                top: Val::Percent(SPEED_INDICATOR_TOP),
                left: Val::Percent(SPEED_INDICATOR_LEFT),
                align_self: AlignSelf::Center,
                ..default()
            },
            ImageNode::from_atlas_image(
                speed_indicator_texture,
                TextureAtlas::from(speed_indicator_atlas.clone()),
            ),
            SpeedIndicator,
            GameUI::Default,
        ))
        .id()
}