use std::{any::Any, time::Duration};

use bevy::{prelude::*, window::CursorGrabMode};

use crate::prelude::*;

struct PlayerInputContext<'w> {
    mouse: &'w ButtonInput<MouseButton>,
    keyboard: &'w ButtonInput<KeyCode>,
    player: &'w Player,
    voxel_assets: &'w VoxelMap,
    current_ui: &'w GameUI,
    time: &'w Time,
    has_window: bool,
    place_timer: &'w mut Timer,
    remove_timer: &'w mut Timer,
    event_writer: EventWriter<'w, GameEvent>,
    audio_writer: EventWriter<'w, AudioEvent>,
    logic_writer: EventWriter<'w, LogicEvent>,
}

impl<'w, 's> PlayerInputContext<'w> {
    fn new(
        mouse: &'w ButtonInput<MouseButton>,
        keyboard: &'w ButtonInput<KeyCode>,
        player: &'w Player,
        voxel_assets: &'w VoxelMap,
        current_ui: &'w GameUI,
        time: &'w Time,
        has_window: bool,
        place_timer: &'w mut Timer,
        remove_timer: &'w mut Timer,
        event_writer: EventWriter<'w, GameEvent>,
        audio_writer: EventWriter<'w, AudioEvent>,
        logic_writer: EventWriter<'w, LogicEvent>,
    ) -> Self {
        Self {
            mouse,
            keyboard,
            player,
            voxel_assets,
            current_ui,
            time,
            has_window,
            place_timer,
            remove_timer,
            event_writer,
            audio_writer,
            logic_writer,
        }
    }
    fn process(&mut self) {
        self.handle_world_interactions();

        if !self.has_window {
            // Mirror the previous behaviour by skipping UI shortcuts when the window is absent.
            return;
        }

        self.handle_ui_shortcuts();
    }
    
    fn handle_ui_shortcuts(&mut self) {
        if self.keyboard.just_pressed(KeyCode::Escape) {
            if *self.current_ui != GameUI::ExitMenu {
                self.set_ui(GameUI::ExitMenu, CursorGrabMode::None, true, false);
            } else {
                self.set_ui(GameUI::Default, CursorGrabMode::Locked, false, true);
            }
            return;
        }

        if self.keyboard.just_pressed(KeyCode::Tab) {
            if *self.current_ui != GameUI::ExitMenu {
                let selector = self.player.hotbar_selector;
                let size = SUBSET_SIZES[selector];
                self.set_ui(GameUI::Inventory(size), CursorGrabMode::Locked, true, false);
            }
            return;
        }

        if self.keyboard.just_released(KeyCode::Tab) {
            if *self.current_ui != GameUI::ExitMenu {
                self.set_ui(GameUI::Default, CursorGrabMode::Locked, false, true);
            }
        }

        if self.keyboard.just_pressed(KeyCode::F3) {
            let new_ui = if *self.current_ui == GameUI::Debug {
                GameUI::Default
            } else {
                GameUI::Debug
            };
            self.switch_ui(new_ui);
        }

        if self.keyboard.just_pressed(KeyCode::Comma) {
            self.event_writer
                .send(GameEvent::SpeedChange { change: -1 });
        } else if self.keyboard.just_pressed(KeyCode::Period) {
            self.event_writer.send(GameEvent::SpeedChange { change: 1 });
        }
    }

    fn handle_world_interactions(&mut self) {
        if *self.current_ui != GameUI::Default && *self.current_ui != GameUI::ClockWidget {
            return;
        }
        if *self.current_ui == GameUI::Default { // Stops certain interactions while not in the default UI state
            self.handle_block_placement();
            self.handle_block_removal();
            self.handle_hotbar_copy();

        }
        self.handle_block_interaction();
    }

    fn handle_block_placement(&mut self) {
        if !mouse_triggered(
            self.place_timer,
            MouseButton::Left,
            self.mouse,
            self.time,
            PLAYER_PLACE_DELAY,
        ) {
            return;
        }

        if let Some(mut selected_voxel) = self.player.selected_voxel {
            self.audio_writer
                .send(AudioEvent::World(WorldSfx::Place, selected_voxel.position));

            selected_voxel.direction = cardinalize(self.player.camera_dir);

            let voxel_asset = self.voxel_assets.asset_map[&selected_voxel.kind].clone();

            self.event_writer.send(GameEvent::PlaceBlock {
                voxel: selected_voxel,
                voxel_asset,
            });

            let mesh_updates = get_neighboring_coords(selected_voxel.position);
            self.event_writer.send(GameEvent::UpdateMesh {
                updates: mesh_updates,
            });
        }
    }

    fn handle_block_removal(&mut self) {
        if !mouse_triggered(
            self.remove_timer,
            MouseButton::Right,
            self.mouse,
            self.time,
            PLAYER_REMOVE_DELAY,
        ) {
            return;
        }

        if let Some(hit_voxel) = self.player.hit_voxel {
            self.audio_writer
                .send(AudioEvent::World(WorldSfx::Destroy, hit_voxel.position));

            self.event_writer.send(GameEvent::RemoveBlock {
                position: hit_voxel.position,
            });

            let mesh_updates = get_neighboring_coords(hit_voxel.position);
            self.event_writer.send(GameEvent::UpdateMesh {
                updates: mesh_updates,
            });
        }
    }

    fn handle_hotbar_copy(&mut self) {
        if !self.mouse.just_pressed(MouseButton::Middle) {
            return;
        }

        if let Some(hit_voxel) = self.player.hit_voxel {
            let kind = hit_voxel.kind;
            let group = kind.group();

            let mut new_player = Player::clone(self.player);
            if group < new_player.hotbar.len() {
                new_player.hotbar_selector = group;
                new_player.hotbar[group] = kind;

                self.event_writer.send(GameEvent::ModifyPlayer {
                    player_modified: new_player,
                });
            }
        }
    }

    fn handle_block_interaction(&mut self) {
        let pressed = self.keyboard.just_pressed(KeyCode::KeyE);
        let released = self.keyboard.just_released(KeyCode::KeyE);
        
        if released == true { // Fixes a bug where it is difficult to get out of certain UI states
            self.set_ui(GameUI::Default, CursorGrabMode::Locked, false, true);
            println!("Button released");
        }
        
        if !pressed && !released {
            return;
        }

        let Some(voxel) = self.player.hit_voxel else {
            return;
        };
        
        match (voxel.kind, pressed) {
            (VoxelType::Component(ComponentVariants::Switch), true) => {
                let new_state = if voxel.state.is_all_zero() {
                    Bits16::all_ones()
                } else {
                    Bits16::all_zeros()
                };
                self.logic_writer.send(LogicEvent::UpdateVoxel {
                    position: voxel.position,
                    new_state,
                });
            }
            (VoxelType::Component(ComponentVariants::Button), true) => {
                self.logic_writer.send(LogicEvent::UpdateVoxel {
                    position: voxel.position,
                    new_state: Bits16::all_ones(),
                });
            }
            (VoxelType::Component(ComponentVariants::Button), false) => {
                self.logic_writer.send(LogicEvent::UpdateVoxel {
                    position: voxel.position,
                    new_state: Bits16::all_zeros(),
                });
            }
            _ => {}
        }

        
        println!("{}", pressed);
        if voxel.kind == VoxelType::Component(ComponentVariants::Clock) {
            if pressed == true {
                self.set_ui(GameUI::ClockWidget, CursorGrabMode::Locked, true, false);
            } else { 

                self.set_ui(GameUI::Default, CursorGrabMode::Locked, false, true);
            }
        } else {
            println!("Here!");
            return; 

        }

    }

    fn switch_ui(&mut self, new_ui: GameUI) {
        self.event_writer.send(GameEvent::ToggleUI { new_ui });
    }

    fn set_ui(
        &mut self,
        new_ui: GameUI,
        mode: CursorGrabMode,
        show_cursor: bool,
        enable_input: bool,
    ) {
        self.event_writer.send(GameEvent::UpdateCursorMode {
            mode,
            show_cursor,
            enable_input,
        });
        self.event_writer.send(GameEvent::ToggleUI { new_ui });
    }
}

/// Main player input system.
pub fn player_input_system(
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Res<Player>,
    voxel_assets: Res<VoxelMap>,
    mut window_query: Query<&mut Window>,
    event_writer: EventWriter<GameEvent>,
    current_ui: Res<GameUI>,
    mut place_timer: Local<Timer>,
    mut remove_timer: Local<Timer>,
    time: Res<Time>,
    audio_writer: EventWriter<AudioEvent>,
    logic_event_writer: EventWriter<LogicEvent>,
) {
    let has_window = window_query.get_single_mut().is_ok();

    let mut context = PlayerInputContext::new(
        mouse_input.as_ref(),
        keyboard_input.as_ref(),
        player.as_ref(),
        voxel_assets.as_ref(),
        current_ui.as_ref(),
        time.as_ref(),
        has_window,
        &mut place_timer,
        &mut remove_timer,
        event_writer,
        audio_writer,
        logic_event_writer,
    );

    context.process();
}

/// Returns `true` when the provided timer should fire for the given mouse button.
fn mouse_triggered(
    timer: &mut Timer,
    button: MouseButton,
    input: &ButtonInput<MouseButton>,
    time: &Time,
    delay: Duration,
) -> bool {
    if input.just_pressed(button) {
        timer.reset();
        timer.set_duration(delay);
        return true;
    }

    if input.pressed(button) && timer.tick(time.delta()).finished() {
        timer.reset();
        timer.set_duration(delay);
        return true;
    }

    false
}

