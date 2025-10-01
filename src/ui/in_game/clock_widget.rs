use std::process::id;

use bevy_inspector_egui::egui::TextBuffer;

use crate::prelude::*;

const CLOCK_PANEL_WIDTH_PERCENT: f32 = 24.0;
const CLOCK_PANEL_MIN_WIDTH_PX: f32 = 240.0;
const CLOCK_PANEL_MAX_WIDTH_PX: f32 = 320.0;
const CLOCK_PANEL_TOP_PERCENT: f32 = 10.0;
const CLOCK_PANEL_RIGHT_PERCENT: f32 = 38.0;
const CLOCK_PANEL_PADDING_PX: f32 = 18.0;
const CLOCK_PANEL_BORDER_PX: f32 = 2.0;
const CLOCK_PANEL_GAP_PX: f32 = 14.0;

const PAUSE_BUTTON_WIDTH_PERCENT: f32 = 100.0;
const PAUSE_BUTTON_HEIGHT_PERCENT: f32 = 18.0;
const SPEED_BUTTON_WIDTH_PERCENT: f32 = 100.0;
const SPEED_BUTTON_HEIGHT_PERCENT: f32 = 14.0;

const SPEED_SETTINGS: [usize; 4] = [4, 16, 64, 256];

pub fn spawn_clock_widget(commands: &mut Commands) -> Entity {
    let root = spawn_ui_node(
        commands,
        Node {
            width: Val::Percent(CLOCK_PANEL_WIDTH_PERCENT),
            min_width: Val::Px(CLOCK_PANEL_MIN_WIDTH_PX),
            max_width: Val::Px(CLOCK_PANEL_MAX_WIDTH_PX),
            top: Val::Percent(CLOCK_PANEL_TOP_PERCENT),
            right: Val::Percent(CLOCK_PANEL_RIGHT_PERCENT),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            justify_content: JustifyContent::FlexStart,
            padding: UiRect::all(Val::Px(CLOCK_PANEL_PADDING_PX)),
            border: UiRect::all(Val::Px(CLOCK_PANEL_BORDER_PX)),
            row_gap: Val::Px(CLOCK_PANEL_GAP_PX),
            ..default()
        },
        (
            BackgroundColor(Color::linear_rgba(0.08, 0.09, 0.13, 0.92)),
            BorderColor(Color::srgb(0.35, 0.55, 0.9)),
            BorderRadius::all(Val::Px(12.0)),
            BoxShadow {
                color: Color::BLACK.with_alpha(0.45),
                x_offset: Val::Px(0.0),
                y_offset: Val::Px(8.0),
                spread_radius: Val::Px(0.0),
                blur_radius: Val::Px(20.0),
            },
            GameUI::ClockWidget,
        ),
    );
    
    let header = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            Text::new("Clock Speed"),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::srgb(0.92, 0.95, 1.0)),
            TextLayout::new_with_justify(JustifyText::Center),
        ))
        .id();

    commands.entity(header).set_parent(root);
    
    let pause_button = spawn_text_button(
            commands,
            PAUSE_BUTTON_WIDTH_PERCENT,
            PAUSE_BUTTON_HEIGHT_PERCENT,
            "Pause".to_string(),
            MenuAction::ClockSetting(0),
        );
        commands.entity(pause_button).set_parent(root);
        commands.entity(pause_button).insert((
            BackgroundColor(Color::linear_rgba(0.82, 0.33, 0.37, 0.95)),
            BorderRadius::all(Val::Px(10.0)),
        ));
    
    let speed_container = spawn_ui_node(
            commands,
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
            (),
        );   
        

    commands.entity(speed_container).set_parent(root);
    let speed_colors = [
        Color::linear_rgba(0.23, 0.46, 0.75, 0.95),
        Color::linear_rgba(0.24, 0.55, 0.68, 0.95),
        Color::linear_rgba(0.24, 0.6, 0.52, 0.95),
        Color::linear_rgba(0.24, 0.65, 0.38, 0.95),
    ];

    for (speed, color) in SPEED_SETTINGS.into_iter().zip(speed_colors) {
        let button = spawn_text_button(
            commands,
            SPEED_BUTTON_WIDTH_PERCENT,
            SPEED_BUTTON_HEIGHT_PERCENT,
            speed.to_string(),
            MenuAction::ClockSetting(speed),
        );
        commands.entity(button).set_parent(speed_container);
        commands
            .entity(button)
            .insert((BackgroundColor(color), BorderRadius::all(Val::Px(10.0))));
    }
    
    root
}