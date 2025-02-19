use bevy::prelude::*;

use crate::{player::PlayerData, DebugText};

pub fn debug_text(
    mut query: Query<&mut Text, With<DebugText>>,
    entity_query: Query<Entity>,
    player: Res<PlayerData>,
) {
    let entitiy_count = entity_query.iter().count();
    
    let debug_text = format!(
        "
        Camera Pos: {:.1}
        Camera Direction: {:.1}
        Ray Hit: {:.1}
        Selected Block: {:.1}
        Selected Adj.: {:.1}
        Voxel ID: {:?}
        Hotbar: {:?}
        Entity Count: {}
        ", 
        player.camera_pos, 
        player.camera_dir, 
        player.ray_hit_pos, 
        player.selected, 
        player.selected_adjacent,
        player.selector,
        player.hotbar_ids,
        entitiy_count,
    );
    
    for mut text in &mut query {
        text.0 = debug_text.clone();
    }
}