/*
 * This file is part of OpenTTD.
 * OpenTTD is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, version 2.
 * OpenTTD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details. You should have received a copy of the GNU General Public License along with OpenTTD. If not, see <https://www.gnu.org/licenses/old-licenses/gpl-2.0>.
 */

use core::mem;
use enum_bitset::EnumBitset;

/** @file bridge_type.h Header file for bridge types. */

///< Bridge spec number.
pub type BridgeType = u32;

/**
 * This enum is related to the definition of bridge pieces,
 * which is used to determine the proper sprite table to use
 * while drawing a given bridge part.
 */
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum BridgePieces {
    PieceNorth = 0,
    PieceSouth,
    PieceInnerNorth,
    PieceInnerSouth,
    PieceMiddleOdd,
    PieceMiddleEven,
    PieceHead,
}

impl BridgePieces {
    pub const COUNT: usize = mem::variant_count::<BridgePieces>();
}

/** Number of bridge middles pieces. This is all bridge pieces except BRIDGE_PIECE_HEAD. */
pub const NUM_BRIDGE_MIDDLE_PIECES: u32 = (BridgePieces::COUNT - 1) as u32;

/** Obstructed bridge pillars information. */
#[derive(Copy, Clone, EnumBitset)]
#[bitset(name = BridgePillarFlags)]
#[repr(u8)]
pub enum BridgePillarFlag {
    /* Corners are in the same order as Corner enum. */
    ///< West corner is obstructed.
    CornerW,
    ///< South corner is obstructed.
    CornerS,
    ///< East corner is obstructed.
    CornerE,
    ///< North corner is obstructed.
    CornerN,
    /* Edges are in the same order as DiagDirection enum. */
    ///< Northeast edge is obstructed.
    EdgeNE,
    ///< Southeast edge is obstructed.
    EdgeSE,
    ///< Southwest edge is obstructed.
    EdgeSW,
    ///< Northwest edge is obstructed.
    EdgeNW,
}

/** Information about a tile structure that may have a bridge above. */
pub struct BridgeableTileInfo {
    ///< Minimum height for a bridge above. 0 means a bridge is not allowed.
    pub height: u8,
    ///< Disallowed pillar flags for a bridge above
    pub disallowed_pillars: BridgePillarFlags,
}

impl Default for BridgeableTileInfo {
    fn default() -> Self {
        BridgeableTileInfo {
            height: 0,
            disallowed_pillars: BridgePillarFlags::all(),
        }
    }
}
