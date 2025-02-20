mod player;
use player::*;

mod graphics;
use graphics::*;

mod voxel;
use voxel::*;

mod ui;
use ui::*;

mod raycast;
use raycast::*;

mod config;
use config::*;

//mod ui;
//use ui::*;


use bevy::{image::{ImageAddressMode, ImageFilterMode, ImageSamplerDescriptor}, prelude::*, render::mesh::VertexAttributeValues, utils::HashMap};
use bevy_rapier3d::prelude::*;

use bevy_fps_controller::controller::*;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 10000.0,
        })
        .insert_resource(PlayerData::default())
        .insert_resource(ClearColor(Color::linear_rgb(0.83, 0.96, 0.96)))
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
            }
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FpsControllerPlugin)
        .add_systems(Startup, (setup_player, setup, setup_voxel_assets)
        )
        .add_systems(
            Update,
            (cursor_system, respawn_system, raycast_system, debug_text, player_action_system),
        )
        .run();
}

#[derive(Component)]
pub struct DebugText; 

#[derive(Resource)]
pub struct VoxelReasources {
    pub voxel_map: HashMap<IVec3, Entity>,
    texture_atlas: Handle<Image>,
}


pub fn setup(
    mut commands: Commands, 
    mut window: Query<&mut Window>, 
    assets: Res<AssetServer>,     
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut window = window.single_mut();
    window.title = String::from("Bits&Blocks");
    
    let game_resources = VoxelReasources {
        voxel_map: HashMap::new(),
        texture_atlas: assets.load("textures/TexturePack6.png"),
    };
    
    commands.insert_resource(game_resources);
    
    // Spawn Sun 
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 7.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    
    
    let mut mesh: Mesh = (Cuboid::new(WORLD_WIDTH, 1.0, WORLD_WIDTH)).into();
    
    let tiling_factor = 10.0;
    tile_mesh_uvs(&mut mesh, tiling_factor);
    
    let mesh_handle = meshes.add(mesh);
    let image_handle = assets.load("textures/ground.png");
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        perceptual_roughness: 1.0,
        metallic: 0.0,
        ..default()
    });
    
    // Spawn World
    commands.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(material),
        WORLD_TRANSFORM,
    )).insert(Collider::cuboid(WORLD_WIDTH * 0.5, 1.0 *  0.5, WORLD_WIDTH * 0.5));
    
    
    commands.spawn((
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new("hello\nbevy!"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::BLACK),
        // Set the justification of the Text
        TextLayout::new_with_justify(JustifyText::Left),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(5.0),
            right: Val::Percent(5.0),
            ..default()
        },
        DebugText,
    ));
}

fn tile_mesh_uvs(mesh: &mut Mesh, tiling_factor: f32) {
    if let Some(VertexAttributeValues::Float32x2(uvs)) = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
        for uv in uvs.iter_mut() {
            uv[0] *= tiling_factor;
            uv[1] *= tiling_factor;
        }
    }
}