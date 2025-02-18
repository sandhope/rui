// use std::ops::{Deref, DerefMut};

use gpui::{px, App, Global, Hsla, Pixels, SharedString, Window};

use super::ThemeColor;
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
    pub colors: ThemeColor,

    pub appearance: Appearance,
    pub font_family: SharedString,
    pub font_size: Pixels,
    pub radius: Pixels,
    pub shadow: bool,
    pub transparent: Hsla,
    /// Show the scrollbar mode, default: Scrolling
    pub scrollbar_show: ScrollbarShow,
    /// Tile grid size, default is 4px.
    pub tile_grid_size: Pixels,
    /// The shadow of the tile panel.
    pub tile_shadow: bool,
}

impl Theme {
    pub fn toggle_appearance(&mut self, window: &mut Window) {
        self.appearance.toggle();
        self.colors = self.appearance.into();
        window.refresh();
    }

    pub fn set_appearance(&mut self, appearance: Appearance, window: &mut Window) {
        self.appearance = appearance;
        self.colors = ThemeColor::from(appearance);
        window.refresh();
    }

    pub fn set_appearance_light(&mut self, window: &mut Window) {
        self.set_appearance(Appearance::Light, window);
    }

    pub fn set_appearance_dark(&mut self, window: &mut Window) {
        self.set_appearance(Appearance::Dark, window);
    }
}

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

    /// Initializes the [`Theme`] for the application.
    pub fn init(cx: &mut App) {
        Self::sync_system_appearance(cx);
        Self::sync_scrollbar_appearance(cx);
    }

    /// Sync the theme with the system appearance
    pub fn sync_system_appearance(cx: &mut App) {
        Theme::sync_scrollbar_appearance(cx);

        let appearance = Appearance::from(cx.window_appearance());

        if !cx.has_global::<Theme>() {
            let theme = Theme::from(appearance);
            cx.set_global(theme);
        }

        let theme = cx.global_mut::<Theme>();

        theme.appearance = appearance;
        theme.colors = appearance.into();
    }

    /// Sync the Scrollbar showing behavior with the system
    pub fn sync_scrollbar_appearance(cx: &mut App) {
        if cx.should_auto_hide_scrollbars() {
            cx.global_mut::<Theme>().scrollbar_show = ScrollbarShow::Scrolling;
        } else {
            cx.global_mut::<Theme>().scrollbar_show = ScrollbarShow::Always;
        }
    }
}

impl From<Appearance> for Theme {
    fn from(appearance: Appearance) -> Self {
        Theme {
            appearance,
            transparent: Hsla::transparent_black(),
            font_size: px(16.),
            font_family: ".SystemUIFont".into(),
            radius: px(4.),
            shadow: true,
            scrollbar_show: ScrollbarShow::default(),
            tile_grid_size: px(8.),
            tile_shadow: true,
            colors: appearance.into(),
        }
    }
}
