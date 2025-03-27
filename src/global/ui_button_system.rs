use crate::prelude::*;
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_simple_text_input::TextInputSubmitEvent;

pub fn menu_button_system(
    mut query: Query<(&Interaction, &mut BackgroundColor, &MenuButton), Changed<Interaction>>,
    game_ui: ResMut<GameUI>,
    game_save: ResMut<SavedWorld>,
    // other resources like Commands, VoxelMap, Meshes, etc.
    mut exit: EventWriter<AppExit>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    events: EventReader<TextInputSubmitEvent>, // Should probably be moved into the Event Handler
    mut event_writer: EventWriter<GameEvent>,
) {
    for (interaction, mut bg_color, menu_button) in query.iter_mut() {
        // Update button color with our helper.
        update_bg_color(interaction, &mut bg_color);
        if let Interaction::Pressed = *interaction {
            match &menu_button.action {
                MenuAction::LoadWorld(name) => {
                    if name.is_empty() {
                        println!("!!!Empty file names should not be accessible!!!")
                    } else {
                        
                        event_writer.send(GameEvent::ToggleUI { new_ui: GameUI::Default });
                        event_writer.send(GameEvent::LoadWorld { world_name: (game_save.world_name.clone()) });
                        event_writer.send(GameEvent::StateChange { new_state: GameState::InGame });
                    }
                }
                MenuAction::NewGame => {
                    event_writer.send(GameEvent::ToggleUI { new_ui: GameUI::NewGame });
                }
                MenuAction::LoadGame => {
                    event_writer.send(GameEvent::ToggleUI { new_ui: GameUI::LoadGame });
                }
                MenuAction::Options => {
                    event_writer.send(GameEvent::ToggleUI { new_ui: GameUI::Options });
                }
                MenuAction::QuitGame => {
                    exit.send(AppExit::Success); // Woof, should this be an event instead???
                }
                MenuAction::CreateWorld => {
                    println!("Create World {}", game_save.world_name);
                    if game_save.world_name.len() > 0 {
                        event_writer.send(GameEvent::ToggleUI { new_ui: GameUI::Default });
                        event_writer.send(GameEvent::StateChange { new_state: GameState::InGame });
                    
                    } else {
                        *bg_color = Color::linear_rgba(1.0, 0.0, 0.0, 1.0).into();
                    }
                }
                MenuAction::BackToGame => {
                    event_writer.send(GameEvent::ToggleUI { new_ui: GameUI::Default });
                    event_writer.send(GameEvent::UpdateCursor {
                        mode: CursorGrabMode::Locked,
                        show_cursor: false,
                        enable_input: true,
                    });
                }
                MenuAction::MainMenu => {
                    event_writer.send(GameEvent::SaveWorld {
                        world: game_save.clone(),
                    });

                    event_writer.send(GameEvent::StateChange {
                        new_state: GameState::Loading,
                    });
                }
                MenuAction::SaveAndQuit => {
                    event_writer.send(GameEvent::SaveWorld {
                        world: game_save.clone(),
                    });
                    exit.send(AppExit::Success);
                }
                MenuAction::Placeholder => {
                }
                // Handle other actions if needed.
            }
        }
    }
    
    // Shitty code for handling escape in the main menu
    if keyboard_input.just_pressed(KeyCode::Escape) 
    && *game_ui != GameUI::Default
    && *game_ui != GameUI::Inventory
    && *game_ui != GameUI::ExitMenu
    {
        event_writer.send(GameEvent::ToggleUI { new_ui: GameUI::MainScreen });
    }
    
    edit_text_listener(events, game_save);
}
fn update_bg_color(interaction: &Interaction, bg_color: &mut BackgroundColor) {
    *bg_color = match *interaction {
        Interaction::Pressed => Color::linear_rgba(0.0, 1.0, 0.0, 1.0).into(),
        Interaction::Hovered => Color::linear_rgba(1.0, 1.0, 1.0, 1.0).into(),
        Interaction::None => Color::linear_rgba(0.5, 0.5, 0.5, 0.25).into(),
    };
}