/// Scrollbar show mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollbarShow {
    #[default]
    Scrolling,
    Hover,
    Always,
}
