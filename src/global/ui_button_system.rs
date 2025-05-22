use crate::prelude::*;
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_simple_text_input::TextInputSubmitEvent;

/// Handles menu button interactions.
pub fn menu_button_system(
    mut query: Query<(&Interaction, &mut BackgroundColor, &MenuAction), Changed<Interaction>>,
    game_ui: Res<GameUI>,
    game_save: Res<SavedWorld>,
    mut exit: EventWriter<AppExit>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    events: EventReader<TextInputSubmitEvent>, // Consider moving text input handling to its own system.
    mut event_writer: EventWriter<GameEvent>,
    mut audio_writer: EventWriter<AudioEvent>,
    player: Res<Player>,
) {
    for (interaction, mut bg_color, menu_action) in query.iter_mut() {
        // Update button color and play sound based on interaction.
        update_color_audio(interaction, &mut bg_color, &mut audio_writer);

        // Process only when the button is pressed.
        if let Interaction::Pressed = *interaction {
            match menu_action {
                MenuAction::LoadWorld(name) => {
                    if name.is_empty() {
                        // Log an error if an empty file name is encountered.
                        println!("!!!Empty file names should not be accessible!!!");
                    } else {
                        event_writer.send(GameEvent::ToggleUI { new_ui: GameUI::Default });
                        event_writer.send(GameEvent::LoadWorld { world_name: name.clone() });
                        event_writer.send(GameEvent::StateChange { new_state: GameState::InGame });
                    }
                }
                MenuAction::DeleteWorld(name) => {
                    // Delete world and return to load game screen.
                    event_writer.send(GameEvent::DeleteWorld { world_name: name.clone() });
                    event_writer.send(GameEvent::StateChange { new_state: GameState::Loading });
                    event_writer.send(GameEvent::Skip {});
                    event_writer.send(GameEvent::ToggleUI { new_ui: GameUI::LoadGame });
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
                    // Consider refactoring to use an event instead of directly exiting.
                    exit.send(AppExit::Success);
                }
                MenuAction::CreateWorld => {
                    println!("Create World {}", game_save.world_name);
                    if !game_save.world_name.is_empty() {
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
                    event_writer.send(GameEvent::SaveWorld { world: game_save.clone() });
                    event_writer.send(GameEvent::StateChange { new_state: GameState::Loading });
                }
                MenuAction::SaveAndQuit => {
                    event_writer.send(GameEvent::SaveWorld { world: game_save.clone() });
                    exit.send(AppExit::Success);
                }
                MenuAction::InventorySlot(slot_id) => {
                    let selector = player.hotbar_selector;

                    // Clamp the requested sub‑index to what is valid for this group.
                    let subset_len   = SUBSET_SIZES[selector];
                    let clamped_sub  = slot_id.clamp(&0, &(subset_len - 1)).clone();

                    // Build a *new* VoxelType from (group, sub‑index).
                    if let Ok(new_kind) = VoxelType::try_from((selector, clamped_sub)) {
                        let mut updated_player         = player.clone();
                        updated_player.hotbar[selector] = new_kind;

                        event_writer.send(GameEvent::ModifyPlayer {
                            player_modified: updated_player,
                        });
                    } /* else: invalid pair – silently ignore */
                }
            }
        }
    }
    
    // Handle escape key in the main menu to return to the main screen.
    if keyboard_input.just_pressed(KeyCode::Escape)
        && *game_ui != GameUI::Default
        && *game_ui != GameUI::Inventory
        && *game_ui != GameUI::ExitMenu
    {
        event_writer.send(GameEvent::ToggleUI { new_ui: GameUI::MainScreen });
    }
    
    // Delegate text input events to the text listener.
    edit_text_listener(events, event_writer);
}

fn update_color_audio(
    interaction: &Interaction, 
    bg_color: &mut BackgroundColor,
    audio_writer: &mut EventWriter<AudioEvent>,
) {
    *bg_color = match *interaction {
        Interaction::Pressed => {
            audio_writer.send(AudioEvent::UIClick {});
            PRESSED_COLOR.into()
        }
        Interaction::Hovered => {
            audio_writer.send(AudioEvent::UIHover {});
            HOVER_COLOR.into()
        }
        Interaction::None => DEFAULT_COLOR.into(),
    };
}