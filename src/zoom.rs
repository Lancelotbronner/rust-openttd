use enum_bitset::EnumBitset;
use std::mem;
use std::ops::{Add, AddAssign};

/// All zoom levels we know.
// The underlying type is signed so subtract-and-Clamp works without need for casting.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, EnumBitset)]
#[bitset(name = ZoomLevels)]
#[repr(i8)]
pub enum ZoomLevel {
    /// Zoomed 4 times in.
    In4x,
    /// Zoomed 2 times in.
    In2x,
    /// The normal zoom level.
    Normal,
    /// Zoomed 2 times out.
    Out2x,
    /// Zoomed 4 times out.
    Out4x,
    /// Zoomed 8 times out.
    Out8x,
}

impl ZoomLevel {
    /// Minimum zoom level.
    pub const MIN: ZoomLevel = ZoomLevel::In4x;
    /// Maximum zoom level.
    pub const MAX: ZoomLevel = ZoomLevel::Out8x;

    /// All the zoom levels.
    pub const ALL: [ZoomLevel; 6] = [
        ZoomLevel::In4x,
        ZoomLevel::In2x,
        ZoomLevel::Normal,
        ZoomLevel::Out2x,
        ZoomLevel::Out4x,
        ZoomLevel::Out8x,
    ];

    // Here we define in which zoom viewports are

    /// Default zoom level for viewports.
    pub const VIEWPORT: ZoomLevel = ZoomLevel::Normal;
    /// Default zoom level for the news messages.
    pub const NEWS: ZoomLevel = ZoomLevel::Normal;
    ///< Default zoom level for the industry view.
    pub const INDUSTRY: ZoomLevel = ZoomLevel::Normal;
    ///< Default zoom level for the town view.
    pub const TOWN: ZoomLevel = ZoomLevel::Normal;
    ///< Default zoom level for the aircraft view.
    pub const AIRCRAFT: ZoomLevel = ZoomLevel::Normal;
    ///< Default zoom level for the ship view.
    pub const SHIP: ZoomLevel = ZoomLevel::Normal;
    ///< Default zoom level for the train view.
    pub const TRAIN: ZoomLevel = ZoomLevel::Normal;
    ///< Default zoom level for the road vehicle view.
    pub const ROAD_VEHICLE: ZoomLevel = ZoomLevel::Normal;
    ///< Default zoom level for the world screenshot view.
    pub const WORLD_SCREENSHOT: ZoomLevel = ZoomLevel::Normal;

    // Level of detail

    ///< All zoom levels below or equal to this will result in details on the screen, like road-work, ...
    pub const DETAIL: ZoomLevel = ZoomLevel::Out2x;
    /// All zoom levels above this will not show text effects.
    pub const TEXT_EFFECT: ZoomLevel = ZoomLevel::Out2x;

    // Interface

    pub const ZOOM_BASE_SHIFT: usize = ZoomLevel::Normal as usize;
    pub const ZOOM_BASE: usize = 1 << ZoomLevel::ZOOM_BASE_SHIFT;

    pub const MIN_INTERFACE_SCALE: isize = 100;
    pub const MAX_INTERFACE_SCALE: isize = 500;
}

impl TryFrom<i8> for ZoomLevel {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        assert!((ZoomLevel::MIN as i8..=ZoomLevel::MAX as i8).contains(&value));
        Ok(unsafe { mem::transmute(value) })
    }
}

impl Add<i8> for ZoomLevel {
    type Output = ZoomLevel;

    fn add(self, rhs: i8) -> Self::Output {
        ZoomLevel::try_from(rhs).expect("Zoom level out of bounds")
    }
}

impl AddAssign<i8> for ZoomLevel {
    fn add_assign(&mut self, rhs: i8) {
        *self = *self + rhs;
    }
}

pub static mut GUI_SCALE: isize = 0;
pub static mut GUI_SCALE_CFG: isize = 0;

pub static mut GUI_ZOOM: ZoomLevel = ZoomLevel::Normal;
pub static mut FONT_ZOOM: ZoomLevel = ZoomLevel::Normal;

pub const MIN_INTERFACE_SCALE: i32 = 100;
pub const MAX_INTERFACE_SCALE: i32 = 500;
