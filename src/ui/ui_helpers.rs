use bevy::prelude::*;
use crate::prelude::*;
use bevy_simple_text_input::{TextInput, TextInputSettings, TextInputSubmitEvent, TextInputTextColor, TextInputTextFont};

/// A generic helper that spawns a UI node with a given style and additional components.
pub fn spawn_ui_node<B: Bundle>(commands: &mut Commands, style: Node, bundle: B) -> Entity {
    commands.spawn((style, bundle)).id()
}

/// A helper function that creates an ImageNode from an atlas image and sets the atlas index.
pub fn create_atlas_image(
    texture_handle: Handle<Image>,
    button_atlas_handle: Handle<TextureAtlasLayout>,
    index: usize,
) -> ImageNode {
    let mut image_node = ImageNode::from_atlas_image(texture_handle, TextureAtlas::from(button_atlas_handle));
    if let Some(ref mut atlas) = image_node.texture_atlas {
        atlas.index = index;
    }
    image_node
}

/// Spawns the root container node that fills the screen and has a black background.
pub fn spawn_root_node(commands: &mut Commands) -> Entity {
    spawn_ui_node(
        commands,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::Center,
            ..default()
        },
        (BackgroundColor(Color::BLACK), MainMenuEntity),
    )
}

pub fn spawn_popup(
    commands: &mut Commands,
    image_handle: Handle<Image>,
    screen_type: WhichMenuUI,
) -> Entity {
    spawn_ui_node(
        commands,
        Node {
            width: Val::Auto,
            height: Val::Percent(100.0),
            aspect_ratio: Some(16.0 / 9.0),
            flex_wrap: FlexWrap::Wrap,
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            ..default()
        },
        (ImageNode::new(image_handle), Visibility::Hidden, PopUp { screen_type }),
    )
}

/// Creates a sub-node that can be attached as a child (for instance, to hold extra buttons).
pub fn spawn_sub_node(commands: &mut Commands, width: f32, height: f32, bottom: f32) -> Entity {
    spawn_ui_node(
        commands,
        Node {
            width: Val::Percent(width),
            height: Val::Percent(height),
            bottom: Val::Percent(bottom),
            row_gap: Val::Px(8.0),
            align_content: AlignContent::Center,
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
        (), // No extra components
    )
}

/// Create an editable text window
/// Only used in the world creation screen at them moment
pub fn create_editable_text(
    commands: &mut Commands,
) -> Entity {    
    let edit_text = commands
            .spawn((
                Node {
                    width: Val::Percent(25.0),
                    height: Val::Percent(10.0),
                    top: Val::Percent(50.0),
                    border: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                Interaction::None,

                BorderColor(Color::srgb(0.75, 0.52, 0.99)),
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                TextInput,
                TextInputTextFont(TextFont {
                    font_size: 34.,
                    ..default()
                }),
                TextInputTextColor(TextColor(Color::srgb(0.9, 0.9, 0.9))),
                TextInputSettings {
                    retain_on_submit: true,
                    ..default()
                }
            )).id();

    edit_text
}

pub fn edit_text_listener(
    mut events: EventReader<TextInputSubmitEvent>,
    mut save_world: ResMut<SavedWorld>,
) {
    for event in events.read() {
        let unclean_name = event.value.clone();
        let sanitary_name = sanitize_filename(&unclean_name);
        save_world.world_name = sanitary_name;
    }
}

fn sanitize_filename(input: &str) -> String {
    // Define characters that are not allowed in file names
    let invalid_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

    // Filter out invalid characters and control characters
    input
        .chars()
        .filter(|&c| !invalid_chars.contains(&c) && !c.is_control())
        .collect()
}

pub fn spawn_text_button(
    commands: &mut Commands,
    parent: Entity,
    weight: f32,
    height: f32,
    button_index: usize,
    text: String,
) {
    let button_container = spawn_ui_node(
        commands,
        Node {
            width: Val::Percent(weight),
            height: Val::Percent(height),
            justify_content: JustifyContent::Center,
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            margin: UiRect::all(Val::Px(4.0)),
            ..default()
        },
        (Button, BackgroundColor(Color::WHITE), WorldButton { index: button_index, name: text.clone() }),
    );
    
    commands.entity(button_container).set_parent(parent);
    
    let text_node = commands.spawn((
        Text::new(text),
        TextFont {
            font_size: 32.0, 
            ..Default::default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            justify_content: JustifyContent::Center,
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            ..Default::default()
        },
    )).id();
    
    commands.entity(text_node).set_parent(button_container);
}

/// Spawns a button with a container (acting as the border) and an image child.
/// The container includes a BackgroundColor that will update based on user interaction.
/// Spawns a button consisting of a container and an image child.
/// The button container is spawned with default styling, then an image node is attached.
pub fn spawn_button(
    commands: &mut Commands,
    parent: Entity,
    texture_handle: Handle<Image>,
    button_atlas_handle: Handle<TextureAtlasLayout>,
    index: usize,
    height_percent: f32,
) -> Entity {
    // Spawn the button container with a white background.
    let button_container = spawn_ui_node(
        commands,
        Node {
            width: Val::Auto,
            height: Val::Percent(height_percent),
            justify_content: JustifyContent::Center,
            ..default()
        },
        (Button, BackgroundColor(Color::WHITE), ButtonNumber { index }),
    );
    commands.entity(button_container).set_parent(parent);

    // Spawn the child image node with a margin so the container's border shows.
    let image_entity = spawn_ui_node(
        commands,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(90.0),
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            margin: UiRect::all(Val::Px(4.0)),
            ..default()
        },
        create_atlas_image(texture_handle, button_atlas_handle, index),
    );
    commands.entity(image_entity).set_parent(button_container);

    button_container
}

