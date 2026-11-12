use bevy::app::App;
use bevy::color::Color;
use bevy::prelude::{Component, Plugin, Resource};

#[derive(Resource)]
pub struct ThemeColors {
    pub primary: Color,
    pub on_primary: Color,
    pub secondary: Color,
    pub on_secondary: Color,
    pub background: Color,
    pub surface: Color,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            primary: theme::color(theme::BROWN_1),
            secondary: theme::color(theme::BROWN_2),
            on_secondary: theme::color(theme::LIGHT_TAN),
            on_primary: theme::color(theme::LIGHT_BEIGE),
            background: theme::color(theme::GREY_1),
            surface: theme::color(theme::MED_TAN),
        }
    }
}
mod theme {
    use bevy::color::{Color, Srgba};
    use bevy::prelude::error;

    pub const BROWN_1: &str = "#625653";
    pub const BROWN_2: &str = "#443D38";

    pub const TAN: &str = "#9A5D40";

    pub const LIGHT_TAN: &str = "#C2B8A9";

    pub const MED_TAN: &str = "#A98143";

    pub const LIGHT_BEIGE: &str = "#D9DBBA";

    pub const GREY_1: &str = "#6F7777";
    pub const GREY_2: &str = "#989DA0";

    pub const GREY_3: &str = "#3B4262";
    pub const DARKEST: &str = "#2F2A35";

    pub fn srgba(color: &str) -> Srgba {
        let srgba = Srgba::hex(color);
        if srgba.is_err() {
            error!("Failed to parse srgba: {}", color);
            srgba.unwrap()
        } else {
            srgba.unwrap()
        }
    }

    pub fn color(color: &str) -> Color {
        Color::from(srgba(color))
    }
}

#[derive(Component)]
pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ThemeColors::default());
    }
}
