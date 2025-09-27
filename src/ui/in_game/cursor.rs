use crate::prelude::*;

const CROSSHAIR_DEFAULT: &str = "+";
const INTERACT_PROMPT: &str = "E";
const CROSSHAIR_SIZE: f32 = 48.0;


#[derive(Component)]
pub struct Cursor;

#[derive(Component)]
pub struct SpeedIndicator; 

/// Spawns the cursor node at the center of the screen.
pub fn spawn_cursor_node(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Node {
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Text::new(CROSSHAIR_DEFAULT),
            TextFont {
                font_size: CROSSHAIR_SIZE,
                ..default()
            },
            TextColor::BLACK,
            TextLayout::new_with_justify(JustifyText::Center),
            Cursor,
        ))
        .id()
}

pub fn update_cursor(
    mut cursor_query: Query<(&mut Text, &mut TextColor), With<Cursor>>,
    mut speed_indicator_query: Query<&mut ImageNode, With<SpeedIndicator>>,
    simulation_timer: Res<SimulationTimer>,
    player: Res<Player>,
    time: Res<Time>,
) {
    for mut text in &mut cursor_query {
        
        let color_alpha = LinearRgba::new(0.2, 1.0, 0.2, bevy::math::ops::sin(time.elapsed_secs()*6.0)*0.5 + 0.5);
        
        if let Some(voxel) = player.hit_voxel {
            match voxel.kind {
                VoxelType::Component(ComponentVariants::Button) => {
                    text.0.0 = "E".to_string();
                    text.1.0 = Color::LinearRgba(color_alpha);
                }
                VoxelType::Component(ComponentVariants::Switch) => {
                    text.0.0 = "E".to_string();
                    text.1.0 = Color::LinearRgba(color_alpha);
                }
                _ => {
                    text.0.0 = "+".to_string();
                    text.1.0 = Color::BLACK;
                }
            }
        } else {
            text.0.0 = "+".to_string();
            text.1.0 = Color::BLACK;
        }
    }
    
    for mut image in &mut speed_indicator_query {
        if let Some(atlas) = &mut image.texture_atlas {
            atlas.index = simulation_timer.rate as usize;
        }
    }
}


fn crosshair_state(player: &Player, elapsed: f32) -> (&'static str, Color) {
    let highlight = Color::LinearRgba(LinearRgba::new(
        0.2,
        1.0,
        0.2,
        (elapsed * 6.0).sin() * 0.5 + 0.5,
    ));
    match player.hit_voxel.map(|hit| hit.kind) {
        Some(VoxelType::Component(ComponentVariants::Button))
        | Some(VoxelType::Component(ComponentVariants::Switch)) => (INTERACT_PROMPT, highlight),
        _ => (CROSSHAIR_DEFAULT, Color::BLACK),
    }
}