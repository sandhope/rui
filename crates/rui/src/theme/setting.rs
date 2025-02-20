use std::sync::Arc;

use crate::{Appearance, ScrollbarShow, Theme};
use gpui::{App, Global, Window};

/// Customizable settings for the UI and theme system.
#[derive(Clone)]
pub struct ThemeSetting {
    pub active_theme: Arc<Theme>,
    pub theme_mode: ThemeMode,
    pub theme_type: ThemeType,
}

impl Global for ThemeSetting {}

impl ThemeSetting {
    /// Returns the global theme reference
    pub fn global(cx: &App) -> &ThemeSetting {
        cx.global::<ThemeSetting>()
    }

    /// Returns the global theme mutable reference
    pub fn global_mut(cx: &mut App) -> &mut ThemeSetting {
        cx.global_mut::<ThemeSetting>()
    }

    // set ActiveTheme
    pub fn set_active_theme(cx: &mut App, theme: Arc<Theme>, theme_mode: ThemeMode) {
        cx.set_global(ThemeSetting {
            active_theme: theme,
            theme_mode,
            theme_type: ThemeType::Custom,
        });
    }

    /// Toggles the builtin theme current appearance between light and dark.
    /// Updates the color settings based on the new appearance
    /// and refreshes the provided window to reflect the change.
    pub fn toggle_builtin_appearance(&mut self, window: &mut Window) {
        if !self.theme_type.is_builtin() {
            return;
        }
        let mut base_theme = (*self.active_theme).clone();
        base_theme.appearance.toggle();
        base_theme.colors = base_theme.appearance.into();
        self.active_theme = Arc::new(base_theme);
        window.refresh();
    }

    /// Sets the appearance to a specified value.
    /// Updates the theme colors based on the new appearance
    /// and refreshes the provided window to apply the changes.
    ///
    /// # Parameters
    /// - `appearance`: The new appearance to set (light or dark).
    /// - `window`: A mutable reference to the window that needs to be refreshed.
    pub fn set_builtin_appearance(&mut self, appearance: Appearance, window: &mut Window) {
        if !self.theme_type.is_builtin() {
            return;
        }
        let mut base_theme = (*self.active_theme).clone();
        base_theme.appearance = appearance;
        base_theme.colors = appearance.into();
        self.active_theme = Arc::new(base_theme);
        window.refresh();
    }

    /// Initializes the built-in theme based on the provided theme mode.
    pub fn init_builtin_theme(cx: &mut App, theme_mode: ThemeMode) {
        let appearance = match theme_mode {
            ThemeMode::Light => Appearance::Light,
            ThemeMode::Dark => Appearance::Dark,
            ThemeMode::System => Appearance::from(cx.window_appearance()),
        };

        let mut theme = Theme::from(appearance);
        theme.styles.scrollbar_show = if cx.should_auto_hide_scrollbars() {
            ScrollbarShow::Scrolling
        } else {
            ScrollbarShow::Always
        };

        cx.set_global(ThemeSetting {
            active_theme: Arc::new(theme),
            theme_mode,
            theme_type: ThemeType::BuiltIn,
        });
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum ThemeType {
    #[default]
    BuiltIn,
    Custom,
}

impl ThemeType {
    pub fn is_builtin(&self) -> bool {
        matches!(self, ThemeType::BuiltIn)
    }
}

/// The mode use to select a theme.
///
/// `Light` and `Dark` will select their respective themes.
///
/// `System` will select the theme based on the system's appearance.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum ThemeMode {
    /// Use the specified `light` theme.
    Light,
    /// Use the specified `dark` theme.
    Dark,
    /// Use the theme based on the system's appearance.
    #[default]
    System,
}

impl ThemeMode {
    pub fn is_system(&self) -> bool {
        matches!(self, ThemeMode::System)
    }
}
