use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(Resource, Debug, Clone)]
pub struct Player {
    pub camera_pos: Vec3,
    pub camera_dir: Vec3,
    pub ray_hit_pos: Vec3,
    pub hit_voxel: Option<Voxel>,
    pub selected_voxel: Option<Voxel>,
    pub selected_descriptor: Option<VoxelDefinition>,
    pub hotbar_selector: usize,
    pub hotbar: Vec<VoxelType>,      // ← renamed + new type
    pub distance: f32,
}

impl Default for Player {
    fn default() -> Self {
        use VoxelType::*;

        let mut initial_bar_ids: Vec<(usize, usize)> = Vec::new();
        for i in 0..9 {
            initial_bar_ids.push((i, 0));
        }
        
        let mut initial_bar: Vec<VoxelType> = Vec::new();
        for i in 0..9 {
            println!("{:?}", initial_bar_ids[i]);
            let voxel_type = VoxelType::try_from(initial_bar_ids[i]).unwrap();

            initial_bar.push(voxel_type);
        }
        
        Self {
            camera_pos: Vec3::ZERO,
            camera_dir: Vec3::ZERO,
            ray_hit_pos: Vec3::ZERO,
            hit_voxel: None,
            selected_voxel: None,
            selected_descriptor: None,
            hotbar_selector: 0,
            hotbar: initial_bar,
            distance: 0.0,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bits16(u16);

impl Bits16 {
    /* ---------- construction ---------- */
    pub const fn new(value: u16) -> Self { Self(value) }
    pub const fn all_zeros() -> Self { Self(0) }
    pub const fn all_ones()  -> Self { Self(0xFFFF) }

    /* ---------- queries ---------- */
    pub fn get(self, index: u8) -> bool {
        assert!(index < 16, "bit index out of range 0–15");
        (self.0 >> index) & 1 == 1
    }

    /* ---------- mutations ---------- */
    pub fn set   (&mut self, index: u8) { self.0 |=  1 << index; }
    pub fn clear (&mut self, index: u8) { self.0 &= !(1 << index); }
    pub fn toggle(&mut self, index: u8) { self.0 ^=  1 << index; }

    /// Fill every bit with `value` (`true` ⇒ 1, `false` ⇒ 0).
    pub fn fill(&mut self, value: bool) { self.0 = if value { 0xFFFF } else { 0 }; }

    /* ---------- access to raw value ---------- */
    pub const fn value(self) -> u16 { self.0 }
    
    #[inline(always)]
    pub const fn any_set(self) -> bool {
        self.0 != 0
    }

    /// `true` if **all** bits are clear (== 0).
    #[inline(always)]
    pub const fn is_all_zero(self) -> bool {
        self.0 == 0
    }
}

#[inline]
pub fn bitword(b: bool) -> Bits16 {
    if b { Bits16::new(0xFFFF) } else { Bits16::all_zeros() }
}

#[derive(Resource, Clone)]
pub struct VoxelMap {
    pub entity_map: HashMap<IVec3, Entity>, // Entity ids by location
    pub voxel_map: HashMap<IVec3, Voxel>,   // Local voxel values by location
    pub asset_map: HashMap<VoxelType, VoxelAsset>, // global voxel values by VoxelType
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Voxel {
    pub kind: VoxelType,
    pub position: IVec3,
    pub direction: u8,
    pub state: Bits16,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VoxelType {
    Structural(StructuralVariants),
    BundledWire,
    Wire(u8), // Channel
    Not(NotVariants),
    And(AndVariants),
    Or(OrVariants),
    Xor(XorVariants),
    Latch(LatchVariants),
    Component(ComponentVariants),
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StructuralVariants {
    BrownBrick, 
    RedBrick,
    SandstoneBrick,
    StoneBrick,
    RedTile,
    GreenTile,
    BlueTile,
    WhiteTile,
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NotVariants {
    NotGate,
    BufferGate,
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AndVariants {
    AndGate,
    NandGate,
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OrVariants {
    OrGate,
    NorGate,
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum XorVariants {
    XorGate,
    XnorGate,
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LatchVariants{
    DFlipFlop,
    SRLatch,
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComponentVariants{
    Light,
    Switch,
    Button, 
    Clock, 
}

impl VoxelType {
    pub fn group(self) -> usize {
        match self {
            VoxelType::Structural(_) => 0,
            VoxelType::BundledWire => 1, 
            VoxelType::Wire(_) => 2,
            VoxelType::Not(_) => 3,
            VoxelType::And(_) => 4,
            VoxelType::Or(_) => 5,
            VoxelType::Xor(_) => 6,
            VoxelType::Latch(_) => 7,
            VoxelType::Component(_) => 8,
        }
    }
    pub fn sub_group(self) -> usize {
        match self {
            VoxelType::Structural(StructuralVariants::BrownBrick) => 0,
            VoxelType::Structural(StructuralVariants::RedBrick) => 1,
            VoxelType::Structural(StructuralVariants::SandstoneBrick) => 2,
            VoxelType::Structural(StructuralVariants::StoneBrick) => 3,
            VoxelType::Structural(StructuralVariants::RedTile) => 4,
            VoxelType::Structural(StructuralVariants::GreenTile) => 5,
            VoxelType::Structural(StructuralVariants::BlueTile) => 6,
            VoxelType::Structural(StructuralVariants::WhiteTile) => 7,
            
            VoxelType::BundledWire => 0,
            
            VoxelType::Wire(x) => x as usize,
            
            VoxelType::Not(NotVariants::NotGate) => 0,
            VoxelType::Not(NotVariants::BufferGate) => 1,
            
            VoxelType::And(AndVariants::AndGate) => 0,
            VoxelType::And(AndVariants::NandGate) => 1,
            
            VoxelType::Or(OrVariants::OrGate) => 0,
            VoxelType::Or(OrVariants::NorGate) => 1,
            
            VoxelType::Xor(XorVariants::XorGate) => 0,
            VoxelType::Xor(XorVariants::XnorGate) => 1,
            
            VoxelType::Latch(LatchVariants::DFlipFlop) => 0,
            VoxelType::Latch(LatchVariants::SRLatch) => 1,
            
            VoxelType::Component(ComponentVariants::Clock) => 0,
            VoxelType::Component(ComponentVariants::Switch) => 1,
            VoxelType::Component(ComponentVariants::Button) => 2,
            VoxelType::Component(ComponentVariants::Light) => 3,
            _ => 0, 
            
        }
    }
    /// Back to the legacy `(group, subtype)` pair when you need it
    pub fn as_pair(self) -> (usize, usize) {
        (self.group(), self.sub_group())
    }
}

/// Optional: seamless conversion in both directions
impl From<VoxelType> for (usize, usize) {
    fn from(kind: VoxelType) -> Self { kind.as_pair() }
}

impl TryFrom<(usize, usize)> for VoxelType {
    type Error = &'static str;
    fn try_from((g, s): (usize, usize)) -> Result<Self, Self::Error> {
        Ok(match (g, s) {
            (0,0) => VoxelType::Structural(StructuralVariants::BrownBrick),
            (0,1) => VoxelType::Structural(StructuralVariants::RedBrick),
            (0,2) => VoxelType::Structural(StructuralVariants::SandstoneBrick),
            (0,3) => VoxelType::Structural(StructuralVariants::StoneBrick),
            (0,4) => VoxelType::Structural(StructuralVariants::RedTile),
            (0,5) => VoxelType::Structural(StructuralVariants::GreenTile),
            (0,6) => VoxelType::Structural(StructuralVariants::BlueTile),
            (0,7) => VoxelType::Structural(StructuralVariants::WhiteTile),
            
            (1,0) => VoxelType::BundledWire, 
            
            (2,0..=15) => VoxelType::Wire(s as u8),
            
            (3,0) => VoxelType::Not(NotVariants::NotGate),
            (3,1) => VoxelType::Not(NotVariants::BufferGate),
            
            (4,0) => VoxelType::And(AndVariants::AndGate),
            (4,1) => VoxelType::And(AndVariants::NandGate),
            
            (5,0) => VoxelType::Or(OrVariants::OrGate),
            (5,1) => VoxelType::Or(OrVariants::NorGate),
            
            (6,0) => VoxelType::Xor(XorVariants::XorGate),
            (6,1) => VoxelType::Xor(XorVariants::XnorGate),
            
            (7,0) => VoxelType::Latch(LatchVariants::DFlipFlop),
            (7,1) => VoxelType::Latch(LatchVariants::SRLatch),
            
            (8,0) => VoxelType::Component(ComponentVariants::Clock),
            (8,1) => VoxelType::Component(ComponentVariants::Switch),
            (8,2) => VoxelType::Component(ComponentVariants::Button),
            (8,3) => VoxelType::Component(ComponentVariants::Light),
            _                => return Err("Unknown voxel_id"),
        })
    }
}



#[derive(Clone, Debug)]
pub struct VoxelAsset {
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<StandardMaterial>,
    pub definition: VoxelDefinition,
    pub texture_row: usize,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct VoxelDefinition {
    pub voxel_id: VoxelType,
    pub name: String,
}

#[derive(Resource)]
pub struct LoadedSaves {
    pub saves: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Resource, Clone)]
pub struct SavedWorld {
    pub world_name: String,
    pub voxels: Vec<Voxel>,
}

#[derive(Resource, Clone)]
pub struct GameTextures {
    pub ground_texture: Handle<Image>,
    pub cursor_texture: Handle<Image>,
    pub voxel_textures: Handle<Image>,
    pub home_screen_texture: Handle<Image>,
    pub menu_button_texture: Handle<Image>,
    pub new_game_screen_texture: Handle<Image>,
    pub load_game_screen_texture: Handle<Image>,
    pub options_screen_texture: Handle<Image>,
}

#[derive(Resource)]
pub struct AudioHandles {
    pub place: Handle<AudioSource>,
    pub destroy: Handle<AudioSource>,
    pub ui_hover: Handle<AudioSource>,
    pub ui_click: Handle<AudioSource>,
}
