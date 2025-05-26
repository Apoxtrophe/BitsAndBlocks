use crate::prelude::*;

#[derive(Component)]
pub struct Cursor;

/// Spawns the cursor node at the center.
pub fn spawn_cursor_node(
    commands: &mut Commands,
) -> Entity {
    
    let text_node =((Text::new("+"),
        TextFont {
            font_size: 48.0, 
            ..Default::default()
        },
        TextColor::BLACK,
        TextLayout::new_with_justify(JustifyText::Center),
    ));
    
    
    let mut mouse_cursor = commands.spawn((
        Node {
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        text_node,
        Cursor,
    )).id(); 
    
    mouse_cursor
}

pub fn update_cursor(
    mut cursor_query: Query<(&mut Text, &mut TextColor, &Cursor),>,
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
}