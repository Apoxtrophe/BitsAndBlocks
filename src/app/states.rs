use crate::prelude::*;

pub fn register(app: &mut App) {
    register_loading(app);
    register_main_menu(app);
    register_in_game(app);
}

fn register_loading(app: &mut App) {
    app.add_systems(OnEnter(GameState::Loading), loading);
}

fn register_main_menu(app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu);
    app.add_systems(
        Update,
        (update_scroll_position,).run_if(in_state(GameState::MainMenu)),
    );
    app.add_systems(OnExit(GameState::MainMenu), despawn_main_menu);
}

fn register_in_game(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::InGame),
        (setup_player, setup_world, setup_ui),
    );
    app.add_systems(
        Update,
        (
            autosave_system,
            player_input_system,
            respawn_system,
            raycast_system,
            update_debug_text,
            update_hotbar,
            update_inventory,
            update_identifier,
            update_cursor,
            logic_event_handler,
            logic_system,
            update_emissive,
        )
            .run_if(in_state(GameState::InGame)),
    );
    app.add_systems(OnExit(GameState::InGame), despawn_all);
}