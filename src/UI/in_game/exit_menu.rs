use bevy::window::CursorGrabMode;
use crate::prelude::*;

pub fn spawn_exit_menu (
    commands: &mut Commands,
) -> Entity {
    spawn_ui_node(
        commands,
        Node {
            width: Val::Percent(80.0),
            height: Val::Percent(80.0),
            flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        (BackgroundColor(Color::linear_rgba(0.1, 0.1, 0.1, 0.8)), GameUIType { ui: WhichGameUI::ExitMenu }, Visibility::Hidden),
    )
}

pub fn exit_menu_interaction(
    mut query: Query<(&Interaction, &mut BackgroundColor, &ButtonNumber), (Changed<Interaction>, With<Button>)>,
    mut which_ui: ResMut<WhichUIShown>,
    mut event_writer: EventWriter<GameEvent>,
    mut app_state: ResMut<NextState<GameState>>,
) {
    
    for (interaction, mut bg_color, button_number) in query.iter_mut() {
        match *interaction {
            
            Interaction::Pressed => {                
                match button_number.index {
                    8 => {
                        println!("Back To Game");
                        which_ui.ui = WhichGameUI::Default;
                        event_writer.send(GameEvent::UpdateCursor {
                            mode: CursorGrabMode::Locked,
                            show_cursor: false,
                            enable_input: true,
                        });
                    }
                    9 => {
                        println!("Main Menu");

                        
                        app_state.set(GameState::Loading);
                    }
                    10 => {
                        println!("Save & Quit");
                    }
                    11 => {
                        println!("Placeholder");
                    }
                    _ => {}
                }

                *bg_color = Color::linear_rgba(0.0, 1.0, 0.0, 1.0).into();
            }
            Interaction::Hovered => {
                *bg_color = Color::linear_rgba(1.0, 1.0, 1.0, 1.0).into();
            }
            Interaction::None => {
                *bg_color = Color::linear_rgba(0.0, 0.0, 0.0, 0.0).into();
            }
        }
    }
}
