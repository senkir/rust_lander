use bevy::color::Color;
use bevy::ecs::children;
use bevy::picking::hover::Hovered;
use bevy::prelude::{px, Bundle, Node, UiRect, JustifyContent, AlignItems, BorderRadius, FlexDirection, BorderColor, BackgroundColor, Text, TextFont, TextColor, TextShadow, On, PointerOver, Query, With, PointerOut};
use bevy::ui_widgets::Button;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
pub fn button(text: &str) -> impl Bundle {
    (
        Button,
        Hovered::default(),
        Node {
            width: px(150),
            height: px(65),
            border: UiRect::all(px(5)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::MAX,
            ..Default::default()
        },
        BorderColor::all(Color::BLACK),
        BackgroundColor(NORMAL_BUTTON),
        children![(
            Text::new(text),
            // TextFont::default(),
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            // TextShadow::default(),
        )],
    )
}

pub fn on_button_over_style(
    event: On<PointerOver>,
    mut button_qu: Query<&mut BackgroundColor, With<Button>>,
) {
    //TODO: this would change if there were radio buttons
    if let Ok(mut bg_color  ) = button_qu.get_mut(event.entity) {
        *bg_color = HOVERED_BUTTON.into();
    }
}

pub fn on_button_out_style(
    event: On<PointerOut>,
    mut button_qu: Query<&mut BackgroundColor, With<Button>>,
) {
    //TODO: this would change if there were radio buttons
    if let Ok(mut bg_color  ) = button_qu.get_mut(event.entity) {
        *bg_color = NORMAL_BUTTON.into();
    }
}