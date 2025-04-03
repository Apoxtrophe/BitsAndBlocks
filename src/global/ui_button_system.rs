use crate::prelude::*;
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_simple_text_input::TextInputSubmitEvent;

pub fn menu_button_system(
    mut query: Query<(&Interaction, &mut BackgroundColor, &MenuAction), Changed<Interaction>>,
    game_ui: Res<GameUI>,
    game_save: Res<SavedWorld>,
    mut exit: EventWriter<AppExit>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    events: EventReader<TextInputSubmitEvent>, // Should probably be moved into the Event Handler
    mut event_writer: EventWriter<GameEvent>,
    mut audio_writer: EventWriter<AudioEvent>,
    player: Res<Player>,
) {
    for (interaction, mut bg_color, menu_button) in query.iter_mut() {
        // Update button color with our helper.
        update_color_audio(interaction, &mut bg_color, &mut audio_writer);
        if let Interaction::Pressed = *interaction {
            match &menu_button {
                MenuAction::LoadWorld(name) => {
                    if name.is_empty() {
                        println!("!!!Empty file names should not be accessible!!!")
                    } else {
                        //game_save.world_name = name.clone();
                        event_writer.send(GameEvent::ToggleUI { new_ui: GameUI::Default });
                        event_writer.send(GameEvent::LoadWorld { world_name: (name.clone()) });
                        event_writer.send(GameEvent::StateChange { new_state: GameState::InGame });
                    }
                }
                MenuAction::DeleteWorld(name) => {
                    event_writer.send(GameEvent::DeleteWorld { world_name: name.clone() }); // Delete the world from the file system
                    event_writer.send(GameEvent::StateChange { new_state: GameState::Loading }); // Send the user back to loading to reload the saves 
                    event_writer.send(GameEvent::Skip {  });
                    event_writer.send(GameEvent::ToggleUI { new_ui: GameUI::LoadGame }); // Send the user back to the load game screen
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
                MenuAction::Placeholder => {}
                MenuAction::InventorySlot(slot_id) => {
                    let selector = player.hotbar_selector;
                    // Clamp index based on a subset size; extracted clamping logic can be a helper.
                    let mut index: usize = *slot_id;
                    index = index.clamp(0, SUBSET_SIZES[selector] - 1);
                    let mut old_player = player.clone();
                    old_player.hotbar_ids[selector].1 = index;
                    event_writer.send(GameEvent::ModifyPlayer { player_modified: old_player.clone() });
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
    
    edit_text_listener(events, event_writer);
}
fn update_color_audio(
    interaction: &Interaction, 
    bg_color: &mut BackgroundColor,
    audio_writer: &mut EventWriter<AudioEvent>,
) {
    *bg_color = match *interaction {
        Interaction::Pressed =>{
            audio_writer.send(AudioEvent::UIClick {});
            PRESSED_COLOR.into()
        } 
        Interaction::Hovered => {
            audio_writer.send(AudioEvent::UIHover {});
            HOVER_COLOR.into()
        }
        Interaction::None => {
            DEFAULT_COLOR.into()
        }
    };
}