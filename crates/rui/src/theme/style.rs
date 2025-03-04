use gpui::{px, Pixels, SharedString};

use crate::ScrollbarShow;

#[derive(Debug, Clone)]
pub struct ThemeStyles {
    pub font_family: SharedString,
    pub font_size: Pixels,
    pub radius: Pixels,
    pub shadow: bool,
    /// Show the scrollbar mode, default: Scrolling
    pub scrollbar_show: ScrollbarShow,
    /// Tile grid size, default is 4px.
    pub tile_grid_size: Pixels,
    /// The shadow of the tile panel.
    pub tile_shadow: bool,
}

impl Default for ThemeStyles {
    fn default() -> Self {
        Self {
            font_size: px(16.),
            font_family: ".SystemUIFont".into(),
            radius: px(4.),
            shadow: true,
            scrollbar_show: ScrollbarShow::default(),
            tile_grid_size: px(8.),
            tile_shadow: true,
        }
    }
}
