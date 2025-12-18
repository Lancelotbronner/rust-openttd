/*
 * This file is part of OpenTTD.
 * OpenTTD is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, version 2.
 * OpenTTD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details. You should have received a copy of the GNU General Public License along with OpenTTD. If not, see <https://www.gnu.org/licenses/old-licenses/gpl-2.0>.
 */

/* @file linkgraph_type.h Declaration of link graph types used for cargo distribution. */
use enum_bitset::EnumBitset;

pub struct LinkGraphId(u16);

impl LinkGraphId {
    pub const END: LinkGraphId = LinkGraphId(0xFFFF);
    pub const INVALID: LinkGraphId = LinkGraphId(0xFFFF);
}

pub struct LinkGraphJobID(u16);

impl LinkGraphJobID {
    pub const END: LinkGraphJobID = LinkGraphJobID(0xFFFF);
    pub const INVALID: LinkGraphJobID = LinkGraphJobID(0xFFFF);
}

pub struct NodeId(u16);

impl NodeId {
    pub const INVALID: NodeId = NodeId(0xFFFF);
}

pub enum DistributionType {
    ///< Manual distribution. No link graph calculations are run.
    Manual = 0,
    ///< Asymmetric distribution. Usually cargo will only travel in one direction.
    Asymmetric = 1,
    ///< Symmetric distribution. The same amount of cargo travels in each direction between each pair of nodes.
    Symmetric = 2,
}

/**
 * Special modes for updating links. 'Restricted' means that vehicles with
 * 'no loading' orders are serving the link. If a link is only served by
 * such vehicles it's 'fully restricted'. This means the link can be used
 * by cargo arriving in such vehicles, but not by cargo generated or
 * transferring at the source station of the link. In order to find out
 * about this condition we keep two update timestamps in each link, one for
 * the restricted and one for the unrestricted part of it. If either one
 * times out while the other is still valid the link becomes fully
 * restricted or fully unrestricted, respectively.
 * Refreshing a link makes just sure a minimum capacity is kept. Increasing
 * actually adds the given capacity.
 */
#[derive(Clone, Copy, EnumBitset)]
#[bitset(name = EdgeUpdateModes)]
pub enum EdgeUpdateMode {
    ///< Increase capacity.
    Increase,
    ///< Refresh capacity.
    Refresh,
    ///< Use restricted link.
    Restricted,
    ///< Use unrestricted link.
    Unrestricted,
}
