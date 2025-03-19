// ======================================================================
// Module Declarations
// ======================================================================
pub mod prelude;
mod character;
mod event_handling;
mod ui;
mod voxel;
mod loading; 
mod meta; 

// ======================================================================
// External Crate Imports
// ======================================================================
use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSamplerDescriptor};

use bevy_atmosphere::plugin::AtmospherePlugin;
use bevy_rapier3d::prelude::*;
use bevy_fps_controller::controller::*;

use bevy_simple_text_input::TextInputPlugin;
// ======================================================================
// Internal Module Uses
// ======================================================================

pub use prelude::*;

// ======================================================================
// Main Application Setup
// ======================================================================
fn main() {
    App::new()
        // States
        // Resources
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: AMBIENT_LIGHT,
        })
        .insert_resource(Player::default())
        // Events
        .add_event::<GameEvent>()
        // Plugins

        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                address_mode_w: ImageAddressMode::Repeat,
                mag_filter: ImageFilterMode::Nearest,
                min_filter: ImageFilterMode::Linear,
                mipmap_filter: ImageFilterMode::Linear,
                lod_min_clamp: 0.0,
                lod_max_clamp: 0.01,
                ..default()
            },
        }))
        .add_plugins(AtmospherePlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(TextInputPlugin)
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FpsControllerPlugin)
        // Startup Systems
        
        //States
        .init_state::<GameState>()
        // ======================================================================
        // LOADING STATE SYSTEMS
        // ======================================================================
        .add_systems(OnEnter(GameState::Loading), loading)
        // ======================================================================
        // MAINMENU STATE SYSTEMS
        // ======================================================================
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
        
        .add_systems(
            Update,
            (
                menu_interaction_system,
                load_world_button_system, 
                update_pop_window_visibility,
            ).run_if(in_state(GameState::MainMenu))
        )
        
        .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
        
     // ======================================================================
        // INGAME STATE SYSTEMS
        // ======================================================================
        // Setup Systems
        .add_systems(OnEnter(GameState::InGame),
            (
                setup_player, 
                setup_world, 
                setup_ui, 
            ))
        // Update Systems
        .add_systems(
            Update,
            (
                autosave_system,
                input_event_system,
                respawn_system,
                raycast_system,
                update_debug_text,
                update_hotbar,
                update_inventory_ui,
                update_identifier,
                update_game_window_visibility,
                exit_menu_interaction,
                event_handler,
            ).run_if(in_state(GameState::InGame))
        )
        .add_systems(OnExit(GameState::InGame), despawn_all)
        .run();
}

// ======================================================================
// Setup Function
// ======================================================================
pub fn setup_world(
    mut commands: Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    image_handles: Res<GameTextures>,
) {

    // Spawn a directional light (Sun)
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 7.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    )).insert(GameEntity);

    // Create world mesh
    let mut mesh: Mesh = (Cuboid::new(WORLD_WIDTH, 1.0, WORLD_WIDTH)).into();
    let tiling_factor = WORLD_WIDTH;
    tile_mesh_uvs(&mut mesh, tiling_factor);
    let mesh_handle = meshes.add(mesh);

    // Create material with texture
    let image_handle = image_handles.ground_texture.clone();
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        perceptual_roughness: 0.2,
        metallic: 0.2,
        ..default()
    });

    // Spawn the world entity with collider
    commands
        .spawn((
            Mesh3d(mesh_handle),
            MeshMaterial3d(material),
            WORLD_TRANSFORM,
        ))
        .insert(Collider::cuboid(WORLD_WIDTH * 0.5, 0.5, WORLD_WIDTH * 0.5)).insert(GameEntity);
}
