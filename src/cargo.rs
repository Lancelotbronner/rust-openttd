/*
 * This file is part of OpenTTD.
 * OpenTTD is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, version 2.
 * OpenTTD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details. You should have received a copy of the GNU General Public License along with OpenTTD. If not, see <https://www.gnu.org/licenses/old-licenses/gpl-2.0>.
 */

/** @file cargo_type.h Types related to cargoes... */

/** Globally unique label of a cargo type. */
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct CargoLabel(u32);

impl CargoLabel {
    pub const fn new(bytes: &[u8; 4]) -> CargoLabel {
        CargoLabel(u32::from_ne_bytes(*bytes))
    }
}

/**
 * Cargo slots to indicate a cargo type within a game.
 */
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct CargoType(u8);

/*
 * Available types of cargo
 * Labels may be re-used between different climates.
 */

/* Temperate */
pub const CT_PASSENGERS: CargoLabel = CargoLabel::new(b"PASS");
pub const CT_COAL: CargoLabel = CargoLabel::new(b"COAL");
pub const CT_MAIL: CargoLabel = CargoLabel::new(b"MAIL");
pub const CT_OIL: CargoLabel = CargoLabel::new(b"OIL_");
pub const CT_LIVESTOCK: CargoLabel = CargoLabel::new(b"LVST");
pub const CT_GOODS: CargoLabel = CargoLabel::new(b"GOOD");
pub const CT_GRAIN: CargoLabel = CargoLabel::new(b"GRAI");
pub const CT_WOOD: CargoLabel = CargoLabel::new(b"WOOD");
pub const CT_IRON_ORE: CargoLabel = CargoLabel::new(b"IORE");
pub const CT_STEEL: CargoLabel = CargoLabel::new(b"STEL");
pub const CT_VALUABLES: CargoLabel = CargoLabel::new(b"VALU");

/* Arctic */
pub const CT_WHEAT: CargoLabel = CargoLabel::new(b"WHEA");
pub const CT_PAPER: CargoLabel = CargoLabel::new(b"PAPR");
pub const CT_GOLD: CargoLabel = CargoLabel::new(b"GOLD");
pub const CT_FOOD: CargoLabel = CargoLabel::new(b"FOOD");

/* Tropic */
pub const CT_RUBBER: CargoLabel = CargoLabel::new(b"RUBR");
pub const CT_FRUIT: CargoLabel = CargoLabel::new(b"FRUT");
pub const CT_MAIZE: CargoLabel = CargoLabel::new(b"MAIZ");
pub const CT_COPPER_ORE: CargoLabel = CargoLabel::new(b"CORE");
pub const CT_WATER: CargoLabel = CargoLabel::new(b"WATR");
pub const CT_DIAMONDS: CargoLabel = CargoLabel::new(b"DIAM");

/* Toyland */
pub const CT_SUGAR: CargoLabel = CargoLabel::new(b"SUGR");
pub const CT_TOYS: CargoLabel = CargoLabel::new(b"TOYS");
pub const CT_BATTERIES: CargoLabel = CargoLabel::new(b"BATT");
pub const CT_CANDY: CargoLabel = CargoLabel::new(b"SWET");
pub const CT_TOFFEE: CargoLabel = CargoLabel::new(b"TOFF");
pub const CT_COLA: CargoLabel = CargoLabel::new(b"COLA");
pub const CT_COTTON_CANDY: CargoLabel = CargoLabel::new(b"CTCD");
pub const CT_BUBBLES: CargoLabel = CargoLabel::new(b"BUBL");
pub const CT_PLASTIC: CargoLabel = CargoLabel::new(b"PLST");
pub const CT_FIZZY_DRINKS: CargoLabel = CargoLabel::new(b"FZDR");

/** Dummy label for engines that carry no cargo; they actually carry 0 passengers. */
pub const CT_NONE: CargoLabel = CT_PASSENGERS;

/// Invalid cargo type.
pub const CT_INVALID: CargoLabel = CargoLabel(u32::MAX);

/// Original number of cargo types.
pub const NUM_ORIGINAL_CARGO: CargoType = CargoType(12);
/// Maximum number of cargo types in a game.
pub const NUM_CARGO: CargoType = CargoType(64);

/* CARGO_AUTO_REFIT and CARGO_NO_REFIT are stored in save-games for refit-orders, so should not be changed. */
/// Automatically choose cargo type when doing auto refitting.
pub const CARGO_AUTO_REFIT: CargoType = CargoType(0xFD);
/// Do not refit cargo of a vehicle (used in vehicle orders and auto-replace/auto-renew).
pub const CARGO_NO_REFIT: CargoType = CargoType(0xFE);

pub const INVALID_CARGO: CargoType = CargoType(u8::MAX);

/** Mixed cargo types for definitions with cargo that can vary depending on climate. */
#[repr(u8)]
pub enum MixedCargoType {
    /// Cargo can be livestock or fruit.
    LivestockFruit,
    /// Cargo can be grain, wheat or maize.
    GrainWheatMaize,
    /// Cargo can be valuables, gold or diamonds.
    ValuablesGoldDiamonds,
}

/**
 * Special cargo filter criteria.
 * These are used by user interface code only and must not be assigned to any entity. Not all values are valid for every UI filter.
 */
pub mod filter_criteria {
	use crate::cargo::{CargoType, NUM_CARGO};

	/// Show all items independent of carried cargo (i.e. no filtering)
    pub const CF_ANY: CargoType = NUM_CARGO;
    /// Show only items which do not carry cargo (e.g. train engines)
    pub const CF_NONE: CargoType = CargoType(NUM_CARGO.0 + 1);
    /// Show only engines (for rail vehicles only)
    pub const CF_ENGINES: CargoType = CargoType(NUM_CARGO.0 + 2);
    /// Show only vehicles which carry any freight (non-passenger) cargo
    pub const CF_FREIGHT: CargoType = CargoType(NUM_CARGO.0 + 3);

    /// Show items with no rating (station list)
    pub const CF_NO_RATING: CargoType = CargoType(NUM_CARGO.0 + 4);
    /// Select all items (station list)
    pub const CF_SELECT_ALL: CargoType = CargoType(NUM_CARGO.0 + 5);
    /// Expand list to show all items (station list)
    pub const CF_EXPAND_LIST: CargoType = CargoType(NUM_CARGO.0 + 6);
}

/** Test whether cargo type is not INVALID_CARGO */
pub fn is_valid_cargo_type(cargo: CargoType) -> bool {
    cargo != INVALID_CARGO
}

pub struct CargoTypes(u64);

pub const ALL_CARGOTYPES: CargoTypes = CargoTypes(u64::MAX);

/** Class for storing amounts of cargo */
pub struct CargoArray([u32; NUM_CARGO.0 as usize]);

impl CargoArray {
    /**
     * Get the sum of all cargo amounts.
     * @return The sum.
     */
    pub fn sum(&self) -> u32 {
        self.0.iter().sum()
    }

    /**
     * Get the amount of cargos that have an amount.
     * @return The amount.
     */
    pub fn count(&self) -> u32 {
        self.0.iter().filter(|x| **x != 0u32).count() as u32
    }
}
