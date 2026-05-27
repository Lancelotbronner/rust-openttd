/*
 * This file is part of OpenTTD.
 * OpenTTD is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, version 2.
 * OpenTTD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details. You should have received a copy of the GNU General Public License along with OpenTTD. If not, see <https://www.gnu.org/licenses/old-licenses/gpl-2.0>.
 */
use enum_bitset::EnumBitset;

/** @file road_type.h Enums and other types related to roads. */

pub struct RoadTypeLabel(pub u32);

impl RoadTypeLabel {
    pub const fn new(bytes: &[u8; 4]) -> Self {
        Self(u32::from_ne_bytes(*bytes))
    }

    pub const ROAD: RoadTypeLabel = RoadTypeLabel::new(b"ROAD");
    pub const TRAM: RoadTypeLabel = RoadTypeLabel::new(b"ELRD");
}

/**
 * The different roadtypes we support
 */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = RoadTypes)]
pub enum RoadType {
    /// Basic road type
    Road,
    /// Trams
    Tram,
}
// DECLARE_INCREMENT_DECREMENT_OPERATORS(RoadType)

/**
 * The different types of road type.
 */
pub enum RoadTramType {
    /// Road type.
    Road,
    /// Tram type.
    Tram,
}

/**
 * Enumeration for the road parts on a tile.
 *
 * This enumeration defines the possible road parts which
 * can be build on a tile.
 */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = RoadBits)]
pub enum RoadBit {
    /// North-west part
    NW,
    /// South-west part
    SW,
    /// South-east part
    SE,
    /// North-east part
    NE,
}

impl RoadBits {
    /// Full road along the x-axis (south-west + north-east)
    pub const X: RoadBits = RoadBits::from_array([RoadBit::SW, RoadBit::NE]);
    /// Full road along the y-axis (north-west + south-east)
    pub const Y: RoadBits = RoadBits::from_array([RoadBit::NW, RoadBit::SE]);

    /// Road at the two northern edges
    pub const N: RoadBits = RoadBits::from_array([RoadBit::NE, RoadBit::NW]);
    /// Road at the two eastern edges
    pub const E: RoadBits = RoadBits::from_array([RoadBit::NE, RoadBit::SE]);
    /// Road at the two southern edges
    pub const S: RoadBits = RoadBits::from_array([RoadBit::SE, RoadBit::SW]);
    /// Road at the two western edges
    pub const W: RoadBits = RoadBits::from_array([RoadBit::NW, RoadBit::SW]);

    /// Full 4-way crossing
    pub const ALL: RoadBits = RoadBits::all();
}

/** Which directions are disallowed ? */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = DisallowedRoadDirections)]
pub enum DisallowedRoadDirection {
    /// All southbound traffic is disallowed.
    Southbound,
    /// All northbound traffic is disallowed.
    Northbound,
}
