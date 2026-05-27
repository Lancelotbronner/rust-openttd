/*
 * This file is part of OpenTTD.
 * OpenTTD is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, version 2.
 * OpenTTD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details. You should have received a copy of the GNU General Public License along with OpenTTD. If not, see <https://www.gnu.org/licenses/old-licenses/gpl-2.0>.
 */
use enum_bitset::EnumBitset;

/** @file landscape_type.h Types related to the landscape. */

/** Landscape types */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = LandscapeTypes)]
pub enum LandscapeType {
    /// Base landscape.
    Temperate,
    /// Landscape with snow levels.
    Arctic,
    /// Landscape with distinct rainforests and deserts,
    Tropic,
    /// Landscape with funky industries and vehicles.
    Toyland,
}

/**
 * For storing the water borders which shall be retained.
 */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = BorderFlags)]
pub enum BorderFlag {
    /// Border on North East.
    NorthEast,
    /// Border on South East.
    SouthEast,
    /// Border on South West.
    SouthWest,
    /// Border on North West.
    NorthWest,
    /// Randomise borders.
    Random,
}

impl BorderFlags {
    /// Border on all sides.
    pub const ALL: BorderFlags = unsafe { Self::from_repr_unchecked(0b1111) };
}
