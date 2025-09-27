use crate::prelude::*;

const DEBUG_TOP_PERCENT: f32 = 10.0;
const DEBUG_LEFT_PERCENT: f32 = 5.0;
const DEBUG_FONT_SIZE: f32 = 18.0;


/// Spawns the debug text node.
pub fn spawn_debug_text(commands: &mut Commands) -> Entity {
    commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(DEBUG_TOP_PERCENT),
                    left: Val::Percent(DEBUG_LEFT_PERCENT),
                    ..default()
                },
                Text::new("Initializing debug info..."),
                TextFont {
                    font_size: DEBUG_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::BLACK),
                TextLayout::new_with_justify(JustifyText::Left),
                DebugText,
                GameEntity,
                GameUI::Debug,
            ))
            .id()
}

/// Updates the debug text with runtime info.
pub fn update_debug_text(
    mut text_query: Query<&mut Text, With<DebugText>>,
    entity_query: Query<Entity>,
    player: Res<Player>,
    time: Res<Time>,
) {
    let info = DebugInfo::gather(&player, &time, entity_query.iter().count());
    
    for mut text in text_query.iter_mut() {
        text.0 = info.to_string();
    }
}

#[derive(Debug)]
struct DebugInfo<'a> {
    time: &'a Time,
    player: &'a Player,
    entities: usize,
}

impl<'a> DebugInfo<'a> {
    fn gather(player: &'a Player, time: &'a Time, entities: usize) -> Self {
        Self {
            time,
            player,
            entities,
        }
    }
}

impl std::fmt::Display for DebugInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let delta = self.time.delta_secs().max(0.0001);
        let fps = (1.0 / delta).round();
        let elapsed = self.time.elapsed_secs().round();

        writeln!(f, "FPS: {fps:.1}")?;
        writeln!(f, "Delta Time: {delta:.3}s")?;
        writeln!(f, "Elapsed Time: {elapsed:.1}s")?;
        writeln!(f)?;
        writeln!(f, "Camera Pos: {:.1}", self.player.camera_pos)?;
        writeln!(f, "Camera Direction: {:.1}", self.player.camera_dir)?;
        writeln!(f)?;
        writeln!(f, "Ray Hit Pos: {:.1}", self.player.ray_hit_pos)?;
        writeln!(f, "Hit Voxel: {:?}", self.player.hit_voxel)?;
        writeln!(f, "Ray Distance: {:.1}", self.player.distance)?;
        writeln!(f)?;
        writeln!(f, "Selected Voxel: {:?}", self.player.selected_voxel)?;
        writeln!(
            f,
            "Selected Definition: {:?}",
            self.player.selected_descriptor
        )?;
        writeln!(f, "Voxel ID: {:?}", self.player.hotbar_selector)?;
        writeln!(f)?;
        writeln!(f, "Hotbar: {:?}", self.player.hotbar)?;
        write!(f, "Entity Count: {}", self.entities)
    }
}