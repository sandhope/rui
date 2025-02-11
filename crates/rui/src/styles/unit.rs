use gpui::{rems, Rems};

/// The base size of a rem, in pixels.
pub const BASE_REM_SIZE_IN_PX: f32 = 16.;

/// Returns a rem value derived from the provided pixel value and the base rem size (16px).
///
/// This can be used to compute rem values relative to pixel sizes, without
/// needing to hard-code the rem value.
///
/// For instance, instead of writing `rems(0.875)` you can write `rems_from_px(14.)`
#[inline(always)]
pub fn rems_from_px(px: f32) -> Rems {
    rems(px / BASE_REM_SIZE_IN_PX)
}
