use gpui::Rgba;

/// Create an RGBA color from RGB components.
///
/// This function takes three color components (red, green, blue)
/// in the range of 0 to 255 and sets the alpha component to 1.0
/// (fully opaque).
///
/// # Parameters
///
/// - `r`: The red component, represented as an 8-bit unsigned integer.
/// - `g`: The green component, represented as an 8-bit unsigned integer.
/// - `b`: The blue component, represented as an 8-bit unsigned integer.
///
/// # Returns
///
/// An instance of [`Rgba`] with the specified red, green, and blue
/// components, and an alpha of 1.0.
pub fn rgb(r: u8, g: u8, b: u8) -> Rgba {
    Rgba {
        r: (r as f32 / 255.),
        g: (g as f32 / 255.),
        b: (b as f32 / 255.),
        a: 1.,
    }
}

/// Create an RGBA color from RGB components and an alpha value.
///
/// This function takes three color components (red, green, blue)
/// in the range of 0 to 255, and a fourth component for alpha
/// in the range of 0.0 to 1.0 to determine the transparency level.
///
/// # Parameters
///
/// - `r`: The red component, represented as an 8-bit unsigned integer.
/// - `g`: The green component, represented as an 8-bit unsigned integer.
/// - `b`: The blue component, represented as an 8-bit unsigned integer.
/// - `a`: The alpha component, a floating point number in the range [0.0, 1.0].
///
/// # Returns
///
/// An instance of [`Rgba`] with the specified red, green, blue,
/// and alpha components.
pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Rgba {
    Rgba {
        r: (r as f32 / 255.),
        g: (g as f32 / 255.),
        b: (b as f32 / 255.),
        a,
    }
}
