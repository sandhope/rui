use gpui::{hsla, point, BoxShadow, Hsla, Pixels, Rgba};

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

/// Make a [gpui::Hsla] color.
///
/// - h: 0..360.0
/// - s: 0.0..100.0
/// - l: 0.0..100.0
pub fn hsl(h: f32, s: f32, l: f32) -> Hsla {
    hsla(h / 360., s / 100.0, l / 100.0, 1.0)
}

/// Make a BoxShadow like CSS
///
/// e.g:
///
/// If CSS is `box-shadow: 0 0 10px 0 rgba(0, 0, 0, 0.1);`
///
/// Then the equivalent in Rust is
/// - `box_shadow(0., 0., 10., 0., rui::rgba(0, 0, 0, 0.1))`
/// - `box_shadow(0., 0., 10., 0., rui::rgba(0., 0., 0., 0.1))`
/// - `box_shadow(0., 0., 10., 0., gpui::hsla(0., 0., 0., 0.1))`
pub fn box_shadow(
    x: impl Into<Pixels>,
    y: impl Into<Pixels>,
    blur: impl Into<Pixels>,
    spread: impl Into<Pixels>,
    color: impl Into<Hsla>,
) -> BoxShadow {
    BoxShadow {
        offset: point(x.into(), y.into()),
        blur_radius: blur.into(),
        spread_radius: spread.into(),
        color: color.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        let color: Hsla = rgb(84, 169, 255).into();
        println!(
            "hsla({:.2}, {:.2}, {:.2}, {:.2})",
            color.h, color.l, color.s, color.a
        );
    }
}
