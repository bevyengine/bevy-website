use bevy::color::palettes::css::DARK_GREY;
use bevy::color::palettes::tailwind::SLATE_300;
use bevy::input_focus::AutoFocus;
use bevy::input_focus::{
    tab_navigation::{TabGroup, TabIndex, TabNavigationPlugin},
    InputFocus,
};
use bevy::prelude::*;
use bevy::text::{EditableText, TextCursorStyle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TabNavigationPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, text_submission)
        .run();
}

#[derive(Component)]
struct TextOutput;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let root = commands
        .spawn(Node {
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            padding: px(20).all(),
            row_gap: px(16),
            margin: auto().all(),
            ..default()
        })
        .id();

    let text_instructions = commands
        .spawn((
            Text::new("Enter to submit text\nTab to switch inputs"),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
                font_size: FontSize::Px(25.0),
                ..default()
            },
        ))
        .id();

    let text_input_left = build_input_text(&mut commands, true, 24.0);
    let text_input_right = build_input_text(&mut commands, false, 24.0);

    let input_container = commands
        .spawn((
            Node {
                column_gap: px(16),
                ..default()
            },
            AutoFocus,
            TabGroup::new(0),
        ))
        .id();

    // Set up a text output to see the result of our text input
    let text_output = commands
        .spawn((
            Node {
                width: px(400),
                border: px(2).all(),
                padding: px(8).all(),
                ..Default::default()
            },
            BorderColor::from(Color::from(SLATE_300)),
            Text::new(""),
            TextOutput,
            TextLayout {
                linebreak: LineBreak::WordOrCharacter,
                ..default()
            },
            TextFont {
                font_size: FontSize::Px(24.0),
                ..default()
            },
        ))
        .id();

    commands
        .entity(input_container)
        .add_children(&[text_input_left, text_input_right]);

    commands
        .entity(root)
        .add_children(&[text_instructions, input_container, text_output]);
}

fn build_input_text(commands: &mut Commands, is_left: bool, font_size: f32) -> Entity {
    commands
        .spawn((
            Node {
                border: px(2).all(),
                ..Default::default()
            },
            BorderColor::from(Color::from(SLATE_300)),
            Name::new(if is_left { "Left" } else { "Right" }),
            EditableText {
                visible_width: Some(10.),
                allow_newlines: false,
                ..Default::default()
            },
            TextLayout::no_wrap(),
            TextFont {
                font_size: FontSize::Px(font_size),
                ..default()
            },
            TextCursorStyle::default(),
            TabIndex(if is_left { 0 } else { 1 }),
            BackgroundColor(DARK_GREY.into()),
        ))
        .id()
}

// Submit the text when Ctrl+Enter is pressed
fn text_submission(
    input_focus: Res<InputFocus>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut text_input: Query<(&mut EditableText, &Name)>,
    mut text_output: Single<&mut Text, With<TextOutput>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter)
        && let Some(focused_entity) = input_focus.get()
        && let Ok((mut text_input, name)) = text_input.get_mut(focused_entity)
    {
        text_output.0 = format!("{:}: {:}", name, text_input.value());

        text_input.clear();
    }
}
