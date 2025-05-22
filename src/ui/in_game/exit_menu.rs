use crate::prelude::*;

pub fn spawn_exit_menu(
    commands: &mut Commands,
    button_texture: Handle<Image>,
    button_atlas_handle: Handle<TextureAtlasLayout>,
) -> Entity {
    let exit_menu = commands.spawn(exit_menu_bundle()).id();

    let sub_exit_menu = spawn_sub_node(commands, 30.0, 70.0, 15.0);
    commands.entity(sub_exit_menu).set_parent(exit_menu);
    
    let button_options = [
        MenuAction::BackToGame, 
        MenuAction::MainMenu, 
        MenuAction::SaveAndQuit,
    ].to_vec();
    
    for i in 0..button_options.len() {
        spawn_button(
            commands,
            sub_exit_menu,
            button_texture.clone(),
            button_atlas_handle.clone(),
            button_options[i].clone(),
            24.0,
        );
    }
    exit_menu
}

/// Bundles the components for the exit menu.
fn exit_menu_bundle() -> impl Bundle {
    (
        Node {
            width: Val::Percent(40.0),
            height: Val::Percent(80.0),
            top: Val::Percent(10.0),
            border: UiRect::all(Val::Percent(0.2)),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
        BorderColor(Color::WHITE),
        BorderRadius::all(Val::Percent(0.1)),
        BackgroundColor(Color::linear_rgba(0.1, 0.1, 0.1, 0.5)),
        BoxShadow {
            color: Color::BLACK.with_alpha(0.8),
            x_offset: Val::Percent(0.0),
            y_offset: Val::Percent(0.0),
            spread_radius: Val::Percent(1.0),
            blur_radius: Val::Px(1.0),
        },
        GameUI::ExitMenu,
        Visibility::Hidden,
    )
}

