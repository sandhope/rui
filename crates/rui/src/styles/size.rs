use gpui::Pixels;

/// A size for elements.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Size {
    XSmall,
    Small,
    #[default]
    Medium,
    Large,
    Custom(Pixels),
}
