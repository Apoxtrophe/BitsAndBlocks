use bevy::prelude::*;
use crate::prelude::*;

#[derive(Resource, PartialEq, Eq, Clone, Copy)]
pub struct WhichUIShown {
    pub ui: WhichGameUI,
}

/// Resource for keeping track of which main_menu ui is shown
#[derive(Resource, Debug)]
pub struct WhichScreen {
    pub screen: WhichMenuUI,
}
