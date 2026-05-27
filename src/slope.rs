/*
 * This file is part of OpenTTD.
 * OpenTTD is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, version 2.
 * OpenTTD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details. You should have received a copy of the GNU General Public License along with OpenTTD. If not, see <https://www.gnu.org/licenses/old-licenses/gpl-2.0>.
 */

use crate::slope::Slope::{
	SLOPE_ENW, SLOPE_EW, SLOPE_FLAT, SLOPE_NS, SLOPE_NWS, SLOPE_SEN, SLOPE_WSE,
};
use enum_bitset::EnumBitset;

/**
 * @file slope_type.h Definitions of a slope.
 * This file defines the enumeration and helper functions for handling the slope info of a tile.
 */

/**
 * Enumeration of tile corners
 */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = Corners)]
pub enum Corner {
    W = 0,
    S = 1,
    E = 2,
    N = 3,
}

#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = SlopeBits)]
pub enum SlopeBit {
	W, S, E, N, Steep, Halftile, HalftileW, HalftileS, HalftileE, HalftileN,
}

/**
 * Enumeration for the slope-type.
 *
 * This enumeration use the chars N,E,S,W corresponding the
 * direction north, east, south and west. The top corner of a tile
 * is the north-part of the tile. The whole slope is encoded with
 * 5 bits, 4 bits for each corner and 1 bit for a steep-flag.
 *
 * For halftile slopes an extra 3 bits are used to represent this
 * properly; 1 bit for a halftile-flag and 2 bits to encode which
 * extra side (corner) is leveled when the slope of the first 5
 * bits is applied. This means that there can only be one leveled
 * slope for steep slopes, which is logical because two leveled
 * slopes would mean that it is not a steep slope as halftile
 * slopes only span one height level.
 */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = Slopes)]
pub enum Slope {
    /// a flat tile
    SLOPE_FLAT = 0x00,
    /// the west corner of the tile is raised
    SLOPE_W = 0b0001,
    /// the south corner of the tile is raised
    SLOPE_S = 0b0010,
    /// the east corner of the tile is raised
    SLOPE_E = 0b0100,
    /// the north corner of the tile is raised
    SLOPE_N = 0b1000,
    /// indicates the slope is steep
    SLOPE_STEEP = 0x10,
    /// north and west corner are raised
    SLOPE_NW = 0b1001,
    /// south and west corner are raised
    SLOPE_SW = 0b0011,
    /// south and east corner are raised
    SLOPE_SE = 0b0110,
    /// north and east corner are raised
    SLOPE_NE = 0b1100,
    /// east and west corner are raised
    SLOPE_EW = 0b0101,
    /// north and south corner are raised
    SLOPE_NS = 0b1010,
    /// bit mask containing all 'simple' slopes
    SLOPE_ELEVATED = 0b1111,
    /// north, west and south corner are raised
    SLOPE_NWS = 0b1011,
    /// west, south and east corner are raised
    SLOPE_WSE = 0b0111,
    /// south, east and north corner are raised
    SLOPE_SEN = 0b1110,
    /// east, north and west corner are raised
    SLOPE_ENW = 0b1101,
    /// a steep slope falling to east (from west)
    SLOPE_STEEP_W = 0x10 | 0b1011,
    /// a steep slope falling to north (from south)
    SLOPE_STEEP_S = 0x10 | 0b0111,
    /// a steep slope falling to west (from east)
    SLOPE_STEEP_E = 0x10 | 0b1110,
    /// a steep slope falling to south (from north)
    SLOPE_STEEP_N = 0x10 | 0b1101,

    /// one halftile is levelled (non-continuous slope)
    SLOPE_HALFTILE = 0x20,
    /// three bits used for halftile slopes
    SLOPE_HALFTILE_MASK = 0xE0,
    /// the west halftile is levelled (non-continuous slope)
    SLOPE_HALFTILE_W = 0x20 | (0b0001 << 6),
    /// the south halftile is levelled (non-continuous slope)
    SLOPE_HALFTILE_S = 0x20 | (0b0010 << 6),
    /// the east halftile is levelled (non-continuous slope)
    SLOPE_HALFTILE_E = 0x20 | (0b0100 << 6),
    /// the north halftile is levelled (non-continuous slope)
    SLOPE_HALFTILE_N = 0x20 | (0b1000 << 6),
}
// DECLARE_ENUM_AS_BIT_SET(Slope)

impl Slope {
	pub fn corners(self) -> Corners {
		unsafe { Corners::from_repr_masked((self as usize >> 6) as u8) }
	}

	pub fn halftile(self) -> Corners {
		unsafe { Corners::from_repr_masked((self as usize >> 6) as u8) }
	}
}

/** Constant bitset with safe slopes for building a level crossing. */
pub const VALID_LEVEL_CROSSING_SLOPES: Slopes = Slopes::from_array([
    SLOPE_SEN, SLOPE_ENW, SLOPE_NWS, SLOPE_NS, SLOPE_WSE, SLOPE_EW, SLOPE_FLAT,
]);

/**
 * Enumeration for Foundations.
 */
pub enum Foundation {
    /// The tile has no foundation, the slope remains unchanged.
    FOUNDATION_NONE,
    /// The tile is leveled up to a flat slope.
    FOUNDATION_LEVELED,
    /// The tile has an along X-axis inclined foundation.
    FOUNDATION_INCLINED_X,
    /// The tile has an along Y-axis inclined foundation.
    FOUNDATION_INCLINED_Y,
    /// The tile has a steep slope. The lowest corner is raised by a foundation to allow building railroad on the lower halftile.
    FOUNDATION_STEEP_LOWER,

    /* Halftile foundations */
    /// The tile has a steep slope. The lowest corner is raised by a foundation and the upper halftile is leveled.
    FOUNDATION_STEEP_BOTH,
    /// Level west halftile non-continuously.
    FOUNDATION_HALFTILE_W,
    /// Level south halftile non-continuously.
    FOUNDATION_HALFTILE_S,
    /// Level east halftile non-continuously.
    FOUNDATION_HALFTILE_E,
    /// Level north halftile non-continuously.
    FOUNDATION_HALFTILE_N,

    /* Special anti-zig-zag foundations for single horizontal/vertical track */
    /// Foundation for TRACK_BIT_LEFT, but not a leveled foundation.
    FOUNDATION_RAIL_W,
    /// Foundation for TRACK_BIT_LOWER, but not a leveled foundation.
    FOUNDATION_RAIL_S,
    /// Foundation for TRACK_BIT_RIGHT, but not a leveled foundation.
    FOUNDATION_RAIL_E,
    /// Foundation for TRACK_BIT_UPPER, but not a leveled foundation.
    FOUNDATION_RAIL_N,

    /// Used inside "rail_cmd.cpp" to indicate invalid slope/track combination.
    Invalid = 0xFF,
}
