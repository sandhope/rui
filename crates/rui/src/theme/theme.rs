use gpui::{App, Global, SharedString};

use super::ThemeColors;
use super::ThemeStyles;
use crate::Appearance;
use crate::ThemeMode;
use crate::ThemeSetting;
use std::sync::Arc;

pub trait ActiveTheme {
    fn theme(&self) -> &Arc<Theme>;
    fn theme_mut(&mut self) -> &mut ThemeSetting;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Arc<Theme> {
        // Theme::global(self)
        // self.global::<Theme>()

        &self.global::<ThemeSetting>().active_theme
    }

    fn theme_mut(&mut self) -> &mut ThemeSetting {
        // Theme::global_mut(self)
        // self.global_mut::<Theme>()

        // ThemeSetting::global_mut(self)
        self.global_mut::<ThemeSetting>()
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    /// The unique identifier for the theme.
    pub id: String,
    /// The name of the theme.
    pub name: SharedString,
    /// The appearance of the theme (light or dark).
    pub appearance: Appearance,
    pub colors: ThemeColors,
    pub styles: ThemeStyles,
}

impl Theme {
    /// Initializes the [`Theme`] for the application.
    pub fn init(cx: &mut App, theme: Option<Arc<Theme>>, theme_mode: Option<ThemeMode>) {
        let theme_mode = theme_mode.unwrap_or_else(|| ThemeMode::default());
        if let Some(theme) = theme {
            ThemeSetting::set_active_theme(cx, theme, theme_mode);
            return;
        }
        ThemeSetting::init_builtin_theme(cx, theme_mode);
    }
}

// use std::ops::{Deref, DerefMut};

// impl Deref for Theme {
//     type Target = ThemeColor;

//     fn deref(&self) -> &Self::Target {
//         &self.colors
//     }
// }

// impl DerefMut for Theme {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.colors
//     }
// }

impl Global for Theme {}

impl Theme {
    /// Returns the global theme reference
    pub fn global(cx: &App) -> &Theme {
        cx.global::<Theme>()
    }

    /// Returns the global theme mutable reference
    pub fn global_mut(cx: &mut App) -> &mut Theme {
        cx.global_mut::<Theme>()
    }
}

impl From<Appearance> for Theme {
    fn from(appearance: Appearance) -> Self {
        Theme {
            id: String::from(appearance.to_string()),
            name: SharedString::from(appearance.to_string()),
            appearance,
            colors: appearance.into(),
            styles: ThemeStyles::default(),
        }
    }
}
