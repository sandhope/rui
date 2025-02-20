use gpui::{App, Global, SharedString, Window};

use super::ThemeColors;
use super::ThemeStyles;
use crate::Appearance;
use crate::ScrollbarShow;

pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
    fn theme_mut(&mut self) -> &mut Theme;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Theme {
        //Theme::global(self)
        self.global::<Theme>()
    }

    fn theme_mut(&mut self) -> &mut Theme {
        //Theme::global_mut(self)
        self.global_mut::<Theme>()
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
    pub fn init(cx: &mut App, theme: Option<Theme>) {
        if let Some(theme) = theme {
            cx.set_global(theme);
            return;
        }
        Self::sync_system_appearance(cx);
        Self::sync_scrollbar_appearance(cx);
    }

    // set ActiveTheme
    pub fn set(cx: &mut App, theme: Theme) {
        cx.set_global(theme);
    }

    /// Sync the theme with the system appearance
    fn sync_system_appearance(cx: &mut App) {
        if cx.has_global::<Theme>() {
            return;
        }
        let appearance = Appearance::from(cx.window_appearance());
        let theme = Theme::from(appearance);
        cx.set_global(theme);
    }

    /// Sync the Scrollbar showing behavior with the system
    fn sync_scrollbar_appearance(cx: &mut App) {
        if cx.should_auto_hide_scrollbars() {
            cx.global_mut::<Theme>().styles.scrollbar_show = ScrollbarShow::Scrolling;
        } else {
            cx.global_mut::<Theme>().styles.scrollbar_show = ScrollbarShow::Always;
        }
    }
}

impl Theme {
    /// Toggles the current appearance between light and dark.
    /// Updates the color settings based on the new appearance
    /// and refreshes the provided window to reflect the change.
    pub fn toggle_appearance(&mut self, window: &mut Window) {
        self.appearance.toggle();
        self.colors = self.appearance.into();
        window.refresh();
    }

    /// Sets the appearance to a specified value.
    /// Updates the theme colors based on the new appearance
    /// and refreshes the provided window to apply the changes.
    ///
    /// # Parameters
    /// - `appearance`: The new appearance to set (light or dark).
    /// - `window`: A mutable reference to the window that needs to be refreshed.
    pub fn set_appearance(&mut self, appearance: Appearance, window: &mut Window) {
        self.appearance = appearance;
        self.colors = ThemeColors::from(appearance);
        window.refresh();
    }

    /// Sets the appearance to light mode.
    /// This method is a convenience wrapper that calls
    /// `set_appearance` with `Appearance::Light` and refreshes the window.
    ///
    /// # Parameters
    /// - `window`: A mutable reference to the window that needs to be refreshed.
    pub fn set_appearance_light(&mut self, window: &mut Window) {
        self.set_appearance(Appearance::Light, window);
    }

    /// Sets the appearance to dark mode.
    /// This method is a convenience wrapper that calls
    /// `set_appearance` with `Appearance::Dark` and refreshes the window.
    ///
    /// # Parameters
    /// - `window`: A mutable reference to the window that needs to be refreshed.
    pub fn set_appearance_dark(&mut self, window: &mut Window) {
        self.set_appearance(Appearance::Dark, window);
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
