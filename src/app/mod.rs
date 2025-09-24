use crate::prelude::*;

mod setup;
mod states;

pub struct GameAppPlugin;

impl Plugin for GameAppPlugin {
    fn build(&self, app: &mut App) {
        setup::configure(app);
        states::register(app);
    }
}