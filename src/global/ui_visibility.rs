use crate::prelude::*;
use bevy::prelude::*;
/// Updates UI element visibility globally based on the current UI state.
pub fn update_ui_visibility(
    mut query: Query<(&GameUI, &mut Visibility)>,
    current_screen: Res<GameUI>,
) {
    for (ui, mut visibility) in query.iter_mut() {
        let should_be_visible = match (&*current_screen, ui) {
            // For inventory: show only x < y, not equal
            (GameUI::Inventory(y), GameUI::Inventory(x)) => x < y,

            // Exact match for all other UI types
            (a, b) if a == b => true,
            
            (GameUI::Inventory(_), GameUI::Default) => true,
            
            _ => false,
        };
        
        println!("GameUI: {:?}", current_screen);

        *visibility = if should_be_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}