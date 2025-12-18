/*
 * This file is part of OpenTTD.
 * OpenTTD is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, version 2.
 * OpenTTD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details. You should have received a copy of the GNU General Public License along with OpenTTD. If not, see <https://www.gnu.org/licenses/old-licenses/gpl-2.0>.
 */
use crate::zoom::ZoomLevel;
use enum_bitset::EnumBitset;

/** @file viewport_type.h Types related to viewports. */

/** Flags to control how Viewport Strings are rendered. */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = ViewportStringFlags)]
#[repr(u8)]
pub enum ViewportStringFlag {
    ///< Draw using the small font.
    Small,
    ///< Draw an extra text shadow. Should only be used with ViewportStringFlag::Small, as normal font already has a shadow.
    Shadow,
    ///< Draw a colour rect around the sign.
    ColourRect,
    ///< Draw a transparent rect around the sign.
    TransparentRect,
    ///< Draw text in colour.
    TextColour,
}

/**
 * Data structure for viewport, display of a part of the world
 */
pub struct Viewport {
    ///< Screen coordinate left edge of the viewport
    pub left: i32,
    ///< Screen coordinate top edge of the viewport
    pub top: i32,
    ///< Screen width of the viewport
    pub width: i32,
    ///< Screen height of the viewport
    pub height: i32,

    ///< Virtual left coordinate
    pub virtual_left: i32,
    ///< Virtual top coordinate
    pub virtual_top: i32,
    ///< width << zoom
    pub virtual_width: i32,
    ///< height << zoom
    pub virtual_height: i32,

    ///< The zoom level of the viewport.
    pub zoom: ZoomLevel,
    // pub overlay: Rc<LinkGraphOverlay>,
}

/** Location information about a sign as seen on the viewport */
#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct ViewportSign {
    ///< The center position of the sign
    center: i32,
    ///< The top of the sign
    top: i32,
    ///< The width when not zoomed out (normal font)
    width_normal: u16,
    ///< The width when zoomed out (small font)
    width_small: u16,
    // void UpdatePosition(int center, int top, std::string_view str, std::string_view str_small = {});
    // void MarkDirty(ZoomLevel maxzoom = ZoomLevel::Max) const;
}

/*
/** Specialised ViewportSign that tracks whether it is valid for entering into a Kdtree */
struct TrackedViewportSign : ViewportSign {
    bool kdtree_valid = false; ///< Are the sign data valid for use with the _viewport_sign_kdtree?

    auto operator<=>(const TrackedViewportSign &) const = default;

    /**
     * Update the position of the viewport sign.
     * Note that this function hides the base class function.
     */
    void UpdatePosition(int center, int top, std::string_view str, std::string_view str_small = {})
    {
    this->kdtree_valid = true;
    this->ViewportSign::UpdatePosition(center, top, str, str_small);
    }
};
 */

/**
 * Directions of zooming.
 * @see DoZoomInOutWindow
 */
pub enum ZoomStateChange {
    ///< Zoom in (get more detailed view).
    ZOOM_IN,
    ///< Zoom out (get helicopter view).
    ZOOM_OUT,
    ///< Hack, used to update the button status.
    ZOOM_NONE,
}

/**
 * Some values for constructing bounding boxes (BB). The Z positions under bridges are:
 * z=0..5  Everything that can be built under low bridges.
 * z=6     reserved, currently unused.
 * z=7     Z separator between bridge/tunnel and the things under/above it.
 */
///< Everything that can be built under low bridges, must not exceed this Z height.
pub const BB_HEIGHT_UNDER_BRIDGE: i32 = 6;
///< Separates the bridge/tunnel from the things under/above it.
pub const BB_Z_SEPARATOR: i32 = 7;

/** Viewport place method (type of highlighted area and placed objects) */
#[derive(EnumBitset, Copy, Clone)]
pub enum ViewportPlaceMethod {
    ///< drag in X or Y direction
    VPM_X_OR_Y = 0,
    ///< drag only in X axis
    VPM_FIX_X = 1,
    ///< drag only in Y axis
    VPM_FIX_Y = 2,
    ///< area of land in X and Y directions
    VPM_X_AND_Y = 3,
    ///< area of land of limited size
    VPM_X_AND_Y_LIMITED = 4,
    ///< drag only in horizontal direction
    VPM_FIX_HORIZONTAL = 5,
    ///< drag only in vertical direction
    VPM_FIX_VERTICAL = 6,
    ///< Drag only in X axis with limited size
    VPM_X_LIMITED = 7,
    ///< Drag only in Y axis with limited size
    VPM_Y_LIMITED = 8,
    ///< all rail directions
    VPM_RAILDIRS = 0x40,
    ///< similar to VMP_RAILDIRS, but with different cursor
    VPM_SIGNALDIRS = 0x80,
}

/**
 * Drag and drop selection process, or, what to do with an area of land when
 * you've selected it.
 */
pub enum ViewportDragDropSelectionProcess {
    /// Clear area
    DDSP_DEMOLISH_AREA,
    /// Raise / level area
    DDSP_RAISE_AND_LEVEL_AREA,
    /// Lower / level area
    DDSP_LOWER_AND_LEVEL_AREA,
    /// Level area
    DDSP_LEVEL_AREA,
    /// Fill area with desert
    DDSP_CREATE_DESERT,
    /// Fill area with rocks
    DDSP_CREATE_ROCKS,
    /// Create a canal
    DDSP_CREATE_WATER,
    /// Create rivers
    DDSP_CREATE_RIVER,
    /// Plant trees
    DDSP_PLANT_TREES,
    /// Bridge placement
    DDSP_BUILD_BRIDGE,
    /// Build an object
    DDSP_BUILD_OBJECT,

    /* Rail specific actions */
    /// Rail placement
    DDSP_PLACE_RAIL,
    /// Signal placement
    DDSP_BUILD_SIGNALS,
    /// Station placement
    DDSP_BUILD_STATION,
    /// Station removal
    DDSP_REMOVE_STATION,
    /// Rail conversion
    DDSP_CONVERT_RAIL,

    /* Road specific actions */
    /// Road placement (X axis)
    DDSP_PLACE_ROAD_X_DIR,
    /// Road placement (Y axis)
    DDSP_PLACE_ROAD_Y_DIR,
    /// Road placement (auto)
    DDSP_PLACE_AUTOROAD,
    /// Road stop placement (waypoint)
    DDSP_BUILD_ROAD_WAYPOINT,
    /// Road stop placement (buses)
    DDSP_BUILD_BUSSTOP,
    /// Road stop placement (trucks)
    DDSP_BUILD_TRUCKSTOP,
    /// Road stop removal (waypoint)
    DDSP_REMOVE_ROAD_WAYPOINT,
    /// Road stop removal (buses)
    DDSP_REMOVE_BUSSTOP,
    /// Road stop removal (trucks)
    DDSP_REMOVE_TRUCKSTOP,
    /// Road conversion
    DDSP_CONVERT_ROAD,
}

/**
 * Target of the viewport scrolling GS method
 */
pub enum ViewportScrollTarget {
    /// All players
    VST_EVERYONE,
    /// All players in specific company
    VST_COMPANY,
    /// Single player
    VST_CLIENT,
}
