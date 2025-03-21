use bevy::{color::palettes::css::*, window::CursorGrabMode};

use crate::prelude::*;

pub fn spawn_exit_menu (
    commands: &mut Commands,
) -> Entity {
    let box_shadow = commands.spawn(exit_menu_bundle()).id();
    box_shadow
}

fn exit_menu_bundle(

) -> impl Bundle {
    (
        Node {
            width: Val::Percent(50.0),
            height: Val::Percent(90.0),
            border: UiRect::all(Val::Px(16.)),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
        BorderColor(Color::WHITE),
        BorderRadius::all(Val::Percent(0.01)),
        BackgroundColor(Color::linear_rgba(0.1, 0.1, 0.1, 0.5)),
        BoxShadow {
            color: Color::BLACK.with_alpha(0.8),
            x_offset: Val::Percent(0.0),
            y_offset: Val::Percent(0.0),
            spread_radius: Val::Percent(5.0),
            blur_radius: Val::Px(2.0),
        },
        GameUIType { ui: WhichGameUI::ExitMenu },
        Visibility::Hidden,
    )
}

pub fn exit_menu_interaction(
    mut query: Query<(&Interaction, &mut BackgroundColor, &ButtonNumber), (Changed<Interaction>, With<Button>)>,
    mut which_ui: ResMut<WhichUIShown>,
    mut event_writer: EventWriter<GameEvent>,
    mut app_state: ResMut<NextState<GameState>>,
    save_game: Res<SavedWorld>,
    mut exit: EventWriter<AppExit>,
) {
    let saved_world = save_game.clone();
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
                        event_writer.send(GameEvent::SaveWorld { world: saved_world.clone() });
                        
                        event_writer.send(GameEvent::StateChange { new_state: GameState::Loading });
                    }
                    10 => {
                        println!("Save & Quit");
                        event_writer.send(GameEvent::SaveWorld { world: saved_world.clone() });
                        exit.send(AppExit::Success);
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
