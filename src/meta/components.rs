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

/// MarkingComponent for the player inventory
#[derive(Component)]
pub struct InventoryGrid;

/// Marking Component for each hotbar slot, that contains its index 0 - 8
#[derive(Component)]
pub struct HotbarSlot {
    pub index: usize,
}

/// Marking Component for the inventory slot, that contains its index 0 - 15
#[derive(Component)]
pub struct InventorySlot {
    pub index: usize,
}

/// Marking Component for the hotbar identifying text above the hotbar slots.
#[derive(Component)]
pub struct VoxelIdentifierText;

/// Marking component for all game entities in the main scene 
#[derive(Component)]
pub struct MainMenuEntity;

#[derive(Clone, Copy, Debug, Component)]
pub enum ButtonIdentity {
    NewGame,
    LoadGame,
    Options,
    QuitGame,
    CreateWorld,
    BackToGame, // InGame 
    MainMenu, 
    SaveAndQuit,
    Placeholder,
}

/// Marking component for game_save buttons
#[derive(Component)]
pub struct WorldButton {
    pub index: usize,
    pub name: String,
}

#[derive(Resource, Component, PartialEq, Eq, Clone, Copy, Debug)]
pub enum GameUI {
    MainScreen,
    NewGame,
    LoadGame,
    Options,
    // Game UI,
    Default,
    Inventory,
    Hidden, 
    ExitMenu,
    Debug, 
}