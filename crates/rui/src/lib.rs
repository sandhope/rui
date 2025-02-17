mod assets;
mod components;
mod geometry;
mod macros;
mod styles;
mod theme;
mod traits;
mod utils;

pub mod prelude;

pub use assets::*;
pub use components::*;
pub use geometry::*;
pub use prelude::*;
pub use styles::*;
pub use theme::*;
pub use utils::*;

pub fn init(cx: &mut App) {
    theme::init(cx);
}
