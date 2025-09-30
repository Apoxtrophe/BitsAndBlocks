use std::process::id;

use bevy_inspector_egui::egui::TextBuffer;

use crate::prelude::*;

pub fn spawn_clock_widget(commands: &mut Commands) -> Entity {
    // First spawn the root entity and grab its id
    let root = commands
        .spawn((
            Node {
                width: Val::Percent(40.0),
                height: Val::Percent(30.0),
                top: Val::Percent(40.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::WHITE),
            GameUI::ClockWidget,
        ))
        .id();
    
    /*
    // Spawn your button
    let button = spawn_text_button(
        commands,
        10.0,
        10.0,
        "test".to_string(),
        MenuAction::ClockSetting(10),
    );

    // Add it as a child afterwards using `commands.entity`
    commands.entity(root).add_child(button);
     */


    root
}