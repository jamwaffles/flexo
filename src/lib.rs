// #![no_std]

use embedded_graphics_core::geometry::Size;
use embedded_graphics_core::geometry::{Dimensions, OriginDimensions};
use embedded_graphics_core::primitives::Rectangle;

pub struct Box<'a> {
    children: &'a [&'a dyn Dimensions],

    // TODO: Change to an enum(?) with min/max/fixed sizes and/or flex behaviour.
    size: Size,
}

impl<'a> Box<'a> {
    /// Get all items that do not have a fixed size.
    fn flex_items(&self) -> impl Iterator<Item = &&dyn Dimensions> {
        self.children
            .iter()
            .filter(|child| child.bounding_box().size.width > 0)
    }
}

impl OriginDimensions for Box<'_> {
    fn size(&self) -> Size {
        self.size
    }
}

// struct ComputedBox<'a> {
//     item: &'a Box<'a>,
//     pos: Rectangle,
// }

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::{
        prelude::*,
        primitives::{Circle, Rectangle},
    };

    // Container expands to fit all its contents
    #[test]
    fn autoexpand() {
        let sidebar = Circle::new(Point::new(10, 15), 12);
        let main = Rectangle::new(Point::new(1, 2), Size::new(3, 4));

        // Something that expands to fill remaining X space
        let rest = Box {
            children: &[],
            // TODO: Signal dynamic sizing using enum
            size: Size::zero(),
        };

        let rest2 = Box {
            children: &[],
            // TODO: Signal dynamic sizing using enum
            size: Size::zero(),
        };

        let container = Box {
            children: &[&sidebar, &main, &rest, &rest2],
            size: Size::new(50, 60),
        };

        // TODO: impl OriginDimensions for Box
        // Compute space consumed by fixed size items
        let fixed_main_axis_space = container.children.iter().fold(Size::zero(), |acc, child| {
            let bb = child.bounding_box();
            let bb = Rectangle::with_corners(
                Point::zero(),
                bb.bottom_right().unwrap_or_else(Point::zero),
            );

            // Add X sizes, take Y maximum
            // TODO: Generalise to flex horizontal/vertical direction
            Size::new(acc.width + bb.size.width, acc.height.max(bb.size.height))
        });

        let remaining_main_axis_space = container.size - fixed_main_axis_space;

        // TODO: Compute ratios based on flex-grow/flex-shrink
        // Remaining space divided by non-fixed-size elements
        let divided_main_size =
            remaining_main_axis_space.width / container.flex_items().count() as u32;

        assert_eq!(fixed_main_axis_space, Size::new(28, 27));
        assert_eq!(remaining_main_axis_space, Size::new(22, 33));
        assert_eq!(divided_main_size, 11);
    }
}
