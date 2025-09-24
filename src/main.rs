// ======================================================================
// Module Declarations
// ======================================================================
mod app;
mod character;
mod global;
mod loading;
pub mod meta;
pub mod prelude;
pub mod simulation;
mod ui;
mod voxel;

pub use prelude::*;

// ======================================================================
// Main Application Setup
// ======================================================================
fn main() {
    App::new().add_plugins(app::GameAppPlugin).run();
}
