use crate::prelude::*;
use bevy::prelude::*;

/// Updates UI element visibility globally based on the current UI state.
pub fn update_ui_visibility(
    mut query: Query<(&GameUI, &mut Visibility)>,
    current_screen: Res<GameUI>,
) {
    for (ui, mut visibility) in query.iter_mut() {
        // Set visibility based on whether the UI element matches the current screen.
        *visibility = if *ui == *current_screen {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        // Special case: Allow the Default UI (hotbar/inventory) to be visible when in Inventory mode.
        if *ui == GameUI::Default && *current_screen == GameUI::Inventory {
            *visibility = Visibility::Visible;
        }
    }
}