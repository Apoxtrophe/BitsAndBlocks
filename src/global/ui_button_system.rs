use crate::prelude::*;
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_simple_text_input::TextInputSubmitEvent;

fn update_color_audio(
    interaction: &Interaction, 
    bg_color: &mut BackgroundColor,
    audio_writer: &mut EventWriter<AudioEvent>,
) {
    *bg_color = match *interaction {
        Interaction::Pressed => {
            audio_writer.send(AudioEvent::Ui(HudSfx::Click));
            PRESSED_COLOR.into()
        }
        Interaction::Hovered => {
            audio_writer.send(AudioEvent::Ui(HudSfx::Hover));
            HOVER_COLOR.into()
        }
        Interaction::None => DEFAULT_COLOR.into(),
    };
}

/// Handles menu button interactions.
pub fn menu_button_system(
    mut query: Query<(&Interaction, &mut BackgroundColor, &MenuAction), Changed<Interaction>>,
    ui: Res<GameUI>,
    save: Res<SavedWorld>,
    player: Res<Player>,
    mut exit: EventWriter<AppExit>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    events: EventReader<TextInputSubmitEvent>, // Consider moving text input handling to its own system.
    mut event_writer: EventWriter<GameEvent>,
    mut audio_writer: EventWriter<AudioEvent>,

) {
    for (interaction, mut bg_color, menu_action) in query.iter_mut() {
        // Update button color and play sound based on interaction.
        update_color_audio(interaction, &mut bg_color, &mut audio_writer);

        // Process only when the button is pressed.
        if let Interaction::Pressed = *interaction {
            match menu_action {
                MenuAction::LoadWorld(name) if !name.is_empty() => {
                    event_writer.send_batch([
                        GameEvent::ToggleUI { new_ui: GameUI::Default },
                        GameEvent::LoadWorld { world_name: name.clone() },
                        GameEvent::StateChange { new_state: GameState::InGame }
                    ]);
                }
                MenuAction::DeleteWorld(name) => {
                    event_writer.send_batch([
                        GameEvent::DeleteWorld { world_name: name.clone() },
                        GameEvent::StateChange { new_state: GameState::Loading },
                        GameEvent::Skip {},
                        GameEvent::ToggleUI { new_ui: GameUI::LoadGame },
                    ]);
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
                    println!("Create World {}", save.world_name);
                    if !save.world_name.is_empty() {
                        event_writer.send(GameEvent::ToggleUI { new_ui: GameUI::Default });
                        event_writer.send(GameEvent::StateChange { new_state: GameState::InGame });
                    } else {
                        *bg_color = Color::linear_rgba(1.0, 0.0, 0.0, 1.0).into();
                    }
                }
                MenuAction::BackToGame => {
                    event_writer.send(GameEvent::ToggleUI { new_ui: GameUI::Default });
                    event_writer.send(GameEvent::UpdateCursorMode {
                        mode: CursorGrabMode::Locked,
                        show_cursor: false,
                        enable_input: true,
                    });
                }
                MenuAction::MainMenu => {
                    event_writer.send(GameEvent::SaveWorld { world: save.clone() });
                    event_writer.send(GameEvent::StateChange { new_state: GameState::Loading });
                }
                MenuAction::SaveAndQuit => {
                    event_writer.send(GameEvent::SaveWorld { world: save.clone() });
                    exit.send(AppExit::Success);
                }
                MenuAction::InventorySlot(slot_id) => {
                    let sel = player.hotbar_selector;
                    let subset   = SUBSET_SIZES[sel];
                    //let clamped_sub  = slot_id.clamp(&0, &(subset - 1)).clone();
                    let subindex  = (*slot_id).min(subset.saturating_sub(1));
                    
                    if let Ok(new_kind) = VoxelType::try_from((sel, subindex)) {
                        let mut updated_player         = player.clone();
                        updated_player.hotbar[sel] = new_kind;
                        event_writer.send(GameEvent::ModifyPlayer {
                            player_modified: updated_player,
                        });
                    } 
                }
                _ => {}
            }
        }
    }
    
    // Handle escape key in the main menu to return to the main screen.
    if keyboard_input.just_pressed(KeyCode::Escape)
        && *ui != GameUI::Default
        && matches!(*ui, GameUI::Inventory(_))
        && *ui != GameUI::ExitMenu
    {
        event_writer.send(GameEvent::ToggleUI { new_ui: GameUI::MainScreen });
    }
    
    // Delegate text input events to the text listener.
    edit_text_listener(events, event_writer);
}

