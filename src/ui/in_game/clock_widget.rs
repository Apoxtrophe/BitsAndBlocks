use crate::prelude::*;

pub fn spawn_clock_widget(
    commands: &mut Commands,
) -> Entity {

    commands
        .spawn((
            Node {
                width: Val::Percent(10.0),
                height: Val::Percent(10.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::WHITE),
            GameUI::ClockWidget,

        ))
        .id()
}