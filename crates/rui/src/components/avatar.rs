use crate::prelude::*;

use gpui::{img, AbsoluteLength, AnyElement, ImageSource, Img};

/// An element that renders a user avatar with customizable appearance options.
///
/// # Examples
///
/// ```
/// use ui::{Avatar, AvatarShape};
///
/// Avatar::new("path/to/image.png")
///     .shape(AvatarShape::Circle)
///     .grayscale(true)
///     .border_color(gpui::red());
/// ```
#[derive(IntoElement)]
pub struct Avatar {
    base: Div,
    image: Img,
    shape: AvatarShape,
    size: Option<AbsoluteLength>,
    border_color: Option<Hsla>,
    indicator: Option<AnyElement>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum AvatarShape {
    Circle,
    Square,
}

impl Avatar {
    /// Creates a new avatar element with the specified image source.
    pub fn new(src: impl Into<ImageSource>) -> Self {
        Avatar {
            base: div(),
            image: img(src),
            shape: AvatarShape::Circle,
            size: None,
            border_color: None,
            indicator: None,
        }
    }

    /// Sets the shape of the avatar.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::{Avatar, AvatarShape};
    ///
    /// let avatar = Avatar::new("path/to/image.png").shape(AvatarShape::Circle);
    /// ```
    pub fn shape(mut self, shape: AvatarShape) -> Self {
        self.shape = shape;
        self
    }

    /// Applies a grayscale filter to the avatar image.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::{Avatar, AvatarShape};
    ///
    /// let avatar = Avatar::new("path/to/image.png").grayscale(true);
    /// ```
    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.image = self.image.grayscale(grayscale);
        self
    }

    /// Sets the border color of the avatar.
    ///
    /// This might be used to match the border to the background color of
    /// the parent element to create the illusion of cropping another
    /// shape underneath (for example in face piles.)
    pub fn border_color(mut self, color: impl Into<Hsla>) -> Self {
        self.border_color = Some(color.into());
        self
    }

    /// Size overrides the avatar size. By default they are 1rem.
    pub fn size<L: Into<AbsoluteLength>>(mut self, size: impl Into<Option<L>>) -> Self {
        self.size = size.into().map(Into::into);
        self
    }

    /// Sets the current indicator to be displayed on the avatar, if any.
    pub fn indicator<E: IntoElement>(mut self, indicator: impl Into<Option<E>>) -> Self {
        self.indicator = indicator.into().map(IntoElement::into_any_element);
        self
    }
}

impl Styled for Avatar {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Avatar {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let border_width = if self.border_color.is_some() {
            px(2.)
        } else {
            px(0.)
        };

        let image_size = self.size.unwrap_or_else(|| rems(1.).into());
        let container_size = image_size.to_pixels(window.rem_size()) + border_width * 2.;

        self.base
            .size(container_size)
            .when(self.shape == AvatarShape::Circle, |this| {
                this.rounded_full()
            })
            .when_some(self.border_color, |this, color| {
                this.border(border_width).border_color(color)
            })
            .child(
                self.image
                    .size(image_size)
                    .when(self.shape == AvatarShape::Circle, |this| {
                        this.rounded_full()
                    }),
            )
            .children(self.indicator.map(|indicator| div().child(indicator)))
    }
}
