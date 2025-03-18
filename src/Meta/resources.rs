use bevy::prelude::*;
use crate::prelude::*;

#[derive(Resource, PartialEq, Eq, Clone, Copy)]
pub struct WhichUIShown {
    pub ui: WhichGameUI,
}
