use crate::prelude::*;

const EXIT_WIDTH_PERCENT: f32 = 40.0;
const EXIT_HEIGHT_PERCENT: f32 = 80.0;
const EXIT_TOP_PERCENT: f32 = 10.0;
const EXIT_BORDER_PERCENT: f32 = 0.2;
const EXIT_RADIUS_PERCENT: f32 = 0.1;
const EXIT_SHADOW_ALPHA: f32 = 0.8;
const EXIT_BUTTON_FONT: f32 = 24.0;

pub fn spawn_exit_menu(
    commands: &mut Commands,
    button_texture: Handle<Image>,
    button_atlas_handle: Handle<TextureAtlasLayout>,
) -> Entity {
    let exit_menu = commands.spawn(exit_menu_bundle()).id();
    
    let container = spawn_sub_node(commands, 30.0, 70.0, 15.0);
    commands.entity(container).set_parent(exit_menu);

    for action in exit_menu_actions() {
        spawn_button(
            commands,
            container,
            button_texture.clone(),
            button_atlas_handle.clone(),
            action.clone(),
            EXIT_BUTTON_FONT,
        );
    }
    exit_menu
}

fn exit_menu_actions() -> &'static [MenuAction] {
    const ACTIONS: &[MenuAction] = &[
        MenuAction::BackToGame,
        MenuAction::MainMenu,
        MenuAction::SaveAndQuit,
    ];
    ACTIONS
}

/// Bundles the components for the exit menu.
fn exit_menu_bundle() -> impl Bundle {
    (
        Node {
            width: Val::Percent(EXIT_WIDTH_PERCENT),
            height: Val::Percent(EXIT_HEIGHT_PERCENT),
            top: Val::Percent(EXIT_TOP_PERCENT),
            border: UiRect::all(Val::Percent(EXIT_BORDER_PERCENT)),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
        BorderColor(Color::WHITE),
        BorderRadius::all(Val::Percent(EXIT_RADIUS_PERCENT)),
        BackgroundColor(Color::linear_rgba(0.1, 0.1, 0.1, 0.5)),
        BoxShadow {
            color: Color::BLACK.with_alpha(EXIT_SHADOW_ALPHA),
            x_offset: Val::Percent(0.0),
            y_offset: Val::Percent(0.0),
            spread_radius: Val::Percent(1.0),
            blur_radius: Val::Px(1.0),
        },
        GameUI::ExitMenu,
        Visibility::Hidden,
    )
}

