//! The geometry module is a collection of types and traits that
//! can be used to describe common units, concepts, and the relationships
//! between them.

/// Represents the orientation of a layout or geometry.
/// This enum defines two possible directions:
/// - Vertical: Represents the vertical orientation.
/// - Horizontal: Represents the horizontal orientation.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Direction {
    Vertical,
    Horizontal,
}

impl Direction {
    pub fn is_vertical(self) -> bool {
        self == Self::Vertical
    }
}

/// Represents the position relative to a reference point,
/// such as in a graphical layout or user interface.
/// This enum defines four cardinal positions:
/// - Top: Represents the top position.
/// - Right: Represents the right position.
/// - Bottom: Represents the bottom position.
/// - Left: Represents the left position.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Position {
    Top,
    Right,
    Bottom,
    Left,
}

/// Represents the edges and corners in a 2D space.
/// This enum defines eight possible directions and edge-corners:
/// - Top: Represents the top edge.
/// - TopRight: Represents the top-right corner.
/// - Right: Represents the right edge.
/// - BottomRight: Represents the bottom-right corner.
/// - Bottom: Represents the bottom edge.
/// - BottomLeft: Represents the bottom-left corner.
/// - Left: Represents the left edge.
/// - TopLeft: Represents the top-left corner.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum EdgeCorner {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}
