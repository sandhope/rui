use gpui::Rems;

/// A size for elements.
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum Size {
    XSmall,
    Small,
    #[default]
    Medium,
    Large,
    Custom(Rems),
}
