use crate::prelude::*;

/// Tunable layout + styling for the inventory UI.
/// Centralizing these makes future tweaks trivial.
#[derive(Clone, Copy)]
struct InventoryUiConfig {
    // root
    root_size_vmin: f32,
    root_bg_rgba: (f32, f32, f32, f32),

    // slot button
    slot_wh_percent: f32,
    slot_margin_auto: bool,
    slot_bg: Color,

    // icon
    icon_inset_percent: f32, // left/top inset as % of slot
    icon_size_percent: f32,  // width/height as % of slot
}

impl Default for InventoryUiConfig {
    fn default() -> Self {
        Self {
            root_size_vmin: 50.0,
            root_bg_rgba: (0.1, 0.1, 0.1, 0.9),

            slot_wh_percent: 23.0,
            slot_margin_auto: true,
            slot_bg: Color::WHITE,

            icon_inset_percent: 5.0,
            icon_size_percent: 90.0,
        }
    }
}

/// Components for an inventory slot's clickable area.
#[derive(Bundle, Clone)]
struct SlotButtonBundle {
    button: Button,
    node: Node,
    visibility: Visibility,
    background: BackgroundColor,
}

impl SlotButtonBundle {
    fn new(cfg: &InventoryUiConfig) -> Self {
        Self {
            button: Button,
            node: Node {
                width: Val::Percent(cfg.slot_wh_percent),
                height: Val::Percent(cfg.slot_wh_percent),
                margin: if cfg.slot_margin_auto {
                    UiRect::all(Val::Auto)
                } else {
                    UiRect::all(Val::Px(0.0))
                },
                ..Default::default()
            },
            visibility: Visibility::Inherited,
            background: BackgroundColor(cfg.slot_bg),
        }
    }
}

/// Child image that shows the slot's icon from a texture atlas.
#[derive(Bundle, Clone)]
struct SlotIconBundle {
    node: Node,
    image: ImageNode,
}

impl SlotIconBundle {
    fn new(
        cfg: &InventoryUiConfig,
        texture_handle: &Handle<Image>,
        texture_atlas_handle: &Handle<TextureAtlasLayout>,
    ) -> Self {
        Self {
            node: Node {
                left: Val::Percent(cfg.icon_inset_percent),
                top: Val::Percent(cfg.icon_inset_percent),
                width: Val::Percent(cfg.icon_size_percent),
                height: Val::Percent(cfg.icon_size_percent),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            image: ImageNode::from_atlas_image(
                texture_handle.clone(),
                TextureAtlas::from(texture_atlas_handle.clone()),
            ),
        }
    }
}

/// Root container of the inventory UI.
fn spawn_inventory_root(commands: &mut Commands, cfg: &InventoryUiConfig) -> Entity {
    commands
        .spawn((
            Node {
                width: Val::VMin(cfg.root_size_vmin),
                height: Val::VMin(cfg.root_size_vmin),
                margin: UiRect::all(Val::Auto),
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            BackgroundColor(Color::linear_rgba(
                cfg.root_bg_rgba.0,
                cfg.root_bg_rgba.1,
                cfg.root_bg_rgba.2,
                cfg.root_bg_rgba.3,
            )),
            Visibility::Hidden,
            GameUI::Inventory(0),
        ))
        .id()
}

/// Spawns the inventory UI and its slot buttons.
pub fn spawn_inventory(
    commands: &mut Commands,
    texture_handle: &Handle<Image>,
    texture_atlas_handle: &Handle<TextureAtlasLayout>,
) -> Entity {
    let cfg = InventoryUiConfig::default();

    let root = spawn_inventory_root(commands, &cfg);

    // Prebuild the bundles once; we clone per-slot below.
    let slot_button = SlotButtonBundle::new(&cfg);
    let slot_icon = SlotIconBundle::new(&cfg, texture_handle, texture_atlas_handle);

    commands.entity(root).with_children(|ui| {
        for i in 0..INVENTORY_SIZE {
            ui.spawn((
                slot_button.clone(),
                MenuAction::InventorySlot(i),
                GameUI::Inventory(i),
            ))
            .with_children(|slot| {
                slot.spawn((
                    slot_icon.clone(),
                    MenuAction::InventorySlot(i),
                    GameUI::Inventory(i),
                ));
            });
        }
    });

    root
}

/// Refreshes every inventory-slot icon so it shows the sprite that corresponds
/// to the (group, sub-index) pair under the new `VoxelType` enum scheme.
pub fn update_inventory(
    mut image_query: Query<(&MenuAction, &mut ImageNode)>,
    player: Res<Player>,
    voxel_map: Res<VoxelMap>,
) {
    let group = player.hotbar_selector;

    for (action, mut img_node) in image_query.iter_mut() {
        // Only update true slot entries.
        let MenuAction::InventorySlot(slot_id) = action else { continue };

        // Clamp subset index to valid range for the active group.
        let subset = if *slot_id >= SUBSET_SIZES[group] { 0 } else { *slot_id };

        // Compute the voxel kind this slot represents and set its sprite.
        if let Ok(kind) = VoxelType::try_from((group, subset)) {
            if let (Some(atlas), Some(asset)) = (&mut img_node.texture_atlas, voxel_map.asset_map.get(&kind)) {
                atlas.index = asset.texture_row;
            }
        }
    }
}
