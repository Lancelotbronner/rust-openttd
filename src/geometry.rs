//! All geometry types in OpenTTD.

use std::cmp::{max, min};

/**
 * Determine where to position a centred object.
 * @param min The top or left coordinate.
 * @param max The bottom or right coordinate.
 * @param size The height or width of the object to draw.
 * @return Offset of where to position the object.
 */
pub fn centre_bounds(min: i32, max: i32, size: i32) -> i32 {
    (min + max - size + 1) / 2
}

/// A coordinate with two dimensions.
#[derive(Default, PartialEq, Eq, Copy, Clone)]
pub struct Coord2D<T> {
    /// X coordinate.
    pub x: T,
    /// Y coordinate.
    pub y: T,
}

/// A coordinate with three dimensions.
#[derive(Default, PartialEq, Eq, Copy, Clone)]
pub struct Coord3D<T> {
    /// X coordinate.
    pub x: T,
    /// Y coordinate.
    pub y: T,
    /// Z coordinate.
    pub z: T,
}

/// Coordinates of a point in 2D
pub type Point = Coord2D<i32>;

/// Dimensions (a width and height) of a rectangle in 2D
#[derive(Default, PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Debug)]
pub struct Dimension {
    pub width: u32,
    pub height: u32,
}

/// Padding dimensions to apply to each side of a Rect.
#[derive(Default, Copy, Clone)]
struct RectPadding {
    pub left: u8,
    pub top: u8,
    pub right: u8,
    pub bottom: u8,
}

impl RectPadding {
    const ZERO: Self = RectPadding {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    /**
     * Get total horizontal padding of RectPadding.
     * @return total horizontal padding.
     */
    fn horizontal(&self) -> u32 {
        self.left as u32 + self.right as u32
    }

    /**
     * Get total vertical padding of RectPadding.
     * @return total vertical padding.
     */
    fn vertical(&self) -> u32 {
        self.top as u32 + self.bottom as u32
    }
}

/// Specification of a rectangle with absolute coordinates of all edges
struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    /**
     * Get width of Rect.
     * @return width of Rect.
     */
    pub fn width(&self) -> i32 {
        self.right - self.left + 1
    }

    /**
     * Get height of Rect.
     * @return height of Rect.
     */
    pub fn height(&self) -> i32 {
        self.right - self.left + 1
    }

    /**
     * Copy and shrink Rect by s pixels.
     * @param s number of pixels to remove from each side of Rect.
     * @return the new smaller Rect.
     */
    #[must_use]
    pub fn shrink1(&self, s: i32) -> Rect {
        self.shrink4(s, s, s, s)
    }

    /**
     * Copy and shrink Rect by h horizontal and v vertical pixels.
     * @param h number of pixels to remove from left and right sides.
     * @param v number of pixels to remove from top and bottom sides.
     * @return the new smaller Rect.
     */
    #[must_use]
    pub fn shrink2(&self, h: i32, v: i32) -> Rect {
        self.shrink4(h, v, h, v)
    }

    /**
     * Copy and shrink Rect by pixels.
     * @param left number of pixels to remove from left side.
     * @param top number of pixels to remove from top side.
     * @param right number of pixels to remove from right side.
     * @param bottom number of pixels to remove from bottom side.
     * @return the new smaller Rect.
     */
    #[must_use]
    pub fn shrink4(&self, left: i32, top: i32, right: i32, bottom: i32) -> Rect {
        Rect {
            left: self.left + left,
            top: self.top + top,
            right: self.right - right,
            bottom: self.bottom - bottom,
        }
    }

    /**
     * Copy and shrink Rect by a different horizontal and vertical RectPadding.
     * @param horz RectPadding to remove from left and right of Rect.
     * @param vert RectPadding to remove from top and bottom of Rect.
     * @return the new smaller Rect.
     */
    #[must_use]
    pub fn shrink2p(&self, horz: RectPadding, vert: RectPadding) -> Rect {
        Rect {
            left: self.left + horz.left as i32,
            top: self.top + vert.top as i32,
            right: self.right - horz.right as i32,
            bottom: self.bottom - vert.bottom as i32,
        }
    }

    /**
     * Copy and expand Rect by s pixels.
     * @param s number of pixels to add to each side of Rect.
     * @return the new larger Rect.
     */
    #[must_use]
    pub fn expand1(&self, s: i32) -> Rect {
        self.shrink1(-s)
    }

    /**
     * Copy and expand Rect by a RectPadding.
     * @param other RectPadding to add to each side of Rect.
     * @return the new larger Rect.
     */
    #[must_use]
    pub fn expand1p(&self, other: RectPadding) -> Rect {
        Rect {
            left: self.left - other.left as i32,
            top: self.top - other.top as i32,
            right: self.right - other.right as i32,
            bottom: self.bottom - other.bottom as i32,
        }
    }

    /**
     * Copy and translate Rect by x,y pixels.
     * @param x number of pixels to move horizontally.
     * @param y number of pixels to move vertically.
     * @return the new translated Rect.
     */
    #[must_use]
    pub fn translate(&self, x: i32, y: i32) -> Rect {
        self.shrink4(x, y, -x, -y)
    }

    /**
     * Copy Rect and set its width.
     * @param width width in pixels for new Rect.
     * @param end   if set, set width at end of Rect, i.e. on right.
     * @return the new resized Rect.
     */
    #[must_use]
    pub fn with_width(&self, width: i32, end: bool) -> Rect {
        if end {
            self.with_x(self.right - width + 1, self.right)
        } else {
            self.with_x(self.left, self.left + width - 1)
        }
    }

    /**
     * Copy Rect and indent it from its position.
     * @param indent offset in pixels for new Rect.
     * @param end   if set, set indent at end of Rect, i.e. on right.
     * @return the new resized Rect.
     */
    #[must_use]
    pub fn indent(&self, indent: i32, end: bool) -> Rect {
        if end {
            self.with_x(self.left, self.right - indent)
        } else {
            self.with_x(self.left + indent, self.right)
        }
    }

    /**
     * Copy Rect and set its height.
     * @param height height in pixels for new Rect.
     * @param end   if set, set height at end of Rect, i.e. at bottom.
     * @return the new resized Rect.
     */
    #[must_use]
    pub fn with_height(&self, height: i32, end: bool) -> Rect {
        if end {
            self.with_y(self.bottom - height + 1, self.top)
        } else {
            self.with_y(self.top, self.top + height - 1)
        }
    }

    /**
     * Test if a point falls inside this Rect.
     * @param pt the point to test.
     * @return true iff the point falls inside the Rect.
     */
    #[must_use]
    pub fn contains(&self, pt: Point) -> bool {
        (pt.x - self.left) <= (self.right - self.left)
            && (pt.y - self.top) <= (self.bottom - self.top)
    }

    /**
     * Centre a vertical dimension within this Rect.
     * @param height The vertical dimension.
     * @return the new resized Rect.
     */
    #[must_use]
    pub fn centre_to_height(&self, height: i32) -> Rect {
        let top = centre_bounds(self.top, self.bottom, height);
        Rect {
            left: self.left,
            top,
            right: self.right,
            bottom: top + height - 1,
        }
    }

    /**
     * Create a new Rect, replacing the left and right coordiates.
     * @param new_left New left coordinate.
     * @param new_right New right coordinate.
     * @return The new Rect.
     */
    #[must_use]
    pub fn with_x(&self, left: i32, right: i32) -> Rect {
        Rect {
            left,
            top: self.top,
            right,
            bottom: self.bottom,
        }
    }

    /**
     * Create a new Rect, replacing the top and bottom coordiates.
     * @param new_top New top coordinate.
     * @param new_bottom New bottom coordinate.
     * @return The new Rect.
     */
    #[must_use]
    pub fn with_y(&self, top: i32, bottom: i32) -> Rect {
        Rect {
            left: self.left,
            top,
            right: self.right,
            bottom,
        }
    }

    /**
     * Create a new Rect, replacing the left and right coordiates.
     * @param other Rect containing the new left and right coordinates.
     * @return The new Rect.
     */
    #[must_use]
    pub fn with_x2(&self, other: Rect) -> Rect {
        self.with_x(other.left, other.right)
    }

    /**
     * Create a new Rect, replacing the top and bottom coordiates.
     * @param other Rect containing the new top and bottom coordinates.
     * @return The new Rect.
     */
    #[must_use]
    pub fn with_y2(&self, other: Rect) -> Rect {
        self.with_y(other.top, other.bottom)
    }

    /**
     * Check if a rectangle is empty.
     * @param r Rectangle to check.
     * @return True if and only if the rectangle doesn't define space.
     */
    #[must_use]
    pub fn is_empty(&self) -> bool {
        (self.left | self.top | self.right | self.bottom) == 0
    }
}

/**
 * Specification of a rectangle with an absolute top-left coordinate and a
 * (relative) width/height
 */
#[derive(Default)]
pub struct PointDimension {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

/**
 * Compute the bounding rectangle around two rectangles.
 * @param r1 First rectangle.
 * @param r2 Second rectangle.
 * @return The bounding rectangle, the smallest rectangle that contains both arguments.
 */
#[must_use]
pub fn bounding_rect(r1: Rect, r2: Rect) -> Rect {
    /* If either the first or the second is empty, return the other. */
    if r1.is_empty() {
        return r2;
    }
    if r2.is_empty() {
        return r1;
    }
    Rect {
        top: min(r1.top, r2.top),
        bottom: max(r1.bottom, r2.bottom),
        left: min(r1.left, r2.left),
        right: max(r1.right, r2.right),
    }
}
