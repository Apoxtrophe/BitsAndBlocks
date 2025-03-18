use bevy::prelude::*;
use crate::prelude::*;

/// Enum containing the global world states
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    InGame,
}

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

/// Keeps track of which GameUI is being shown. 
#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub struct GameUIType {
    pub ui: WhichGameUI,
}

// Local resource for InGame that keeps track of which toggleable ui is shown. 
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WhichGameUI {
    Default,
    Inventory,
    HotbarHidden,
    ExitMenu,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WhichMenuUI {
    MainScreen,
    NewGame,
    LoadGame,
    Settings,
}


#[derive(Component)]
pub struct MainMenuEntity;

#[derive(Component)]
pub struct ButtonNumber {
    pub index: usize,
}

#[derive(Component, Debug)]
pub struct PopUp {
    pub screen_type: WhichMenuUI,
}