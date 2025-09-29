use bevy::prelude::*;

/// Enum containing the global world states
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    InGame,
}

#[derive(Component)]
pub struct PlayerCamera;

/// Marking component for all game entities that belong to the main scene / will be despawned upon changing state.
#[derive(Component)]
pub struct GameEntity; // Entities that are removed after leaving the game state.

/// Marking component for DebugText in the main scene.
#[derive(Component)]
pub struct DebugText;

/// Marking Component for each hotbar slot, that contains its index 0 - 8
#[derive(Component)]
pub struct HotbarSlot {
    pub index: usize,
}

/// Marking Component for the hotbar identifying text above the hotbar slots.
#[derive(Component)]
pub struct VoxelIdentifierText;

/// Marking component for all game entities in the main scene 
#[derive(Component)]
pub struct MainMenuEntity;

/// Marking component that identifies the function of each button in UI 
#[derive(Clone, Debug, Component)]
pub enum MenuAction {
    NewGame,
    LoadGame,
    LoadWorld(String),
    DeleteWorld(String),
    Options,
    QuitGame,
    CreateWorld,
    BackToGame, // InGame 
    MainMenu, 
    SaveAndQuit,
    InventorySlot(usize),
}

// Marking Component for every UI window
#[derive(Resource, Component, PartialEq, Eq, Clone, Copy, Debug)]
pub enum GameUI {
    MainScreen,
    NewGame,
    LoadGame,
    Options,
    // Game UI,
    Default,
    Inventory(usize),
    Hidden, 
    ExitMenu,
    Debug, 
    ClockWidget, 
}