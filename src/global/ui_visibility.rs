use crate::prelude::*;
use bevy::prelude::*;

// OPERATES GLOBALLY REGARDLESS OF GAME STATE

pub fn update_ui_visibility(
    mut query: Query<(&GameUI, &mut Visibility)>,
    current_screen: Res<GameUI>,
) {
    for (ui, mut visibility) in query.iter_mut() {
        if *ui == *current_screen {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
        // Special case for allowing inventory and hotbar to be shown simultaneously
        if *ui == GameUI::Default && *current_screen == GameUI::Inventory {
            *visibility = Visibility::Visible;
        }
    }
}