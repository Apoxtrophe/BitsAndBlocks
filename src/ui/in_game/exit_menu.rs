use bevy::window::CursorGrabMode;

use crate::prelude::*;

pub fn spawn_exit_menu(
    commands: &mut Commands,
    button_texture: Handle<Image>,
    button_atlas_handle: Handle<TextureAtlasLayout>,
) -> Entity {
    let exit_menu = commands.spawn(exit_menu_bundle()).id();

    let sub_exit_menu = spawn_sub_node(commands, 30.0, 70.0, 15.0);
    commands.entity(sub_exit_menu).set_parent(exit_menu);
    
    let button_options = [
        ButtonIdentity::BackToGame, 
        ButtonIdentity::MainMenu, 
        ButtonIdentity::SaveAndQuit,
        ButtonIdentity::Placeholder,
    ].to_vec();
    
    for i in 0..button_options.len() {
        spawn_button(
            commands,
            sub_exit_menu,
            button_texture.clone(),
            button_atlas_handle.clone(),
            button_options[i],
            24.0,
        );
    }
    exit_menu
}

fn exit_menu_bundle() -> impl Bundle {
    (
        Node {
            width: Val::Percent(40.0),
            height: Val::Percent(80.0),
            top: Val::Percent(10.0),
            border: UiRect::all(Val::Percent(0.2)),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
        BorderColor(Color::WHITE),
        BorderRadius::all(Val::Percent(0.1)),
        BackgroundColor(Color::linear_rgba(0.1, 0.1, 0.1, 0.5)),
        BoxShadow {
            color: Color::BLACK.with_alpha(0.8),
            x_offset: Val::Percent(0.0),
            y_offset: Val::Percent(0.0),
            spread_radius: Val::Percent(1.0),
            blur_radius: Val::Px(1.0),
        },
        GameUI::ExitMenu,
        Visibility::Hidden,
    )
}

pub fn exit_menu_interaction(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonIdentity),
        (Changed<Interaction>, With<Button>),
    >,
    mut which_ui: ResMut<GameUI>,
    mut event_writer: EventWriter<GameEvent>,
    save_game: Res<SavedWorld>,
    mut exit: EventWriter<AppExit>,
) {
    let saved_world = save_game.clone();
    for (interaction, mut bg_color, button_number) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match button_number {
                    ButtonIdentity::BackToGame => {
                        println!("Back To Game");
                        *which_ui = GameUI::Default;
                        event_writer.send(GameEvent::UpdateCursor {
                            mode: CursorGrabMode::Locked,
                            show_cursor: false,
                            enable_input: true,
                        });
                    }
                    ButtonIdentity::MainMenu => {
                        println!("Main Menu");
                        event_writer.send(GameEvent::SaveWorld {
                            world: saved_world.clone(),
                        });

                        event_writer.send(GameEvent::StateChange {
                            new_state: GameState::Loading,
                        });
                    }
                    ButtonIdentity::SaveAndQuit => {
                        println!("Save & Quit");
                        event_writer.send(GameEvent::SaveWorld {
                            world: saved_world.clone(),
                        });
                        exit.send(AppExit::Success);
                    }
                    ButtonIdentity::Placeholder => {
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
