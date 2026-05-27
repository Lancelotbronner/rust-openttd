/*
 * This file is part of OpenTTD.
 * OpenTTD is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, version 2.
 * OpenTTD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details. You should have received a copy of the GNU General Public License along with OpenTTD. If not, see <https://www.gnu.org/licenses/old-licenses/gpl-2.0>.
 */
use enum_bitset::EnumBitset;

/** @file order_type.h Types related to orders. */

///< The index of an order within its current vehicle (not pool related)
pub struct VehicleOrderId(pub u8);

impl VehicleOrderId {
    /** Invalid vehicle order index (sentinel) */
    pub const INVALID: VehicleOrderId = VehicleOrderId(0xFF);
    /** Last valid VehicleOrderID. */
    pub const MAX: VehicleOrderId = VehicleOrderId(0xFF - 1);
}

pub struct OrderListId(pub u16);

pub struct DestinationId(pub u16);
// {
// 	using BaseType = uint16_t;
// 	BaseType value = 0;
//
// 	explicit DestinationID() = default;
// 	constexpr DestinationID(size_t index) : value(static_cast<BaseType>(index)) {}
// 	constexpr DestinationID(DepotID depot) : value(depot.base()) {}
// 	constexpr DestinationID(StationID station) : value(station.base()) {}
//
// 	constexpr DepotID ToDepotID() const noexcept { return static_cast<DepotID>(this->value); }
// 	constexpr StationID ToStationID() const noexcept { return static_cast<StationID>(this->value); }
// 	constexpr BaseType base() const noexcept { return this->value; }
//
// 	constexpr bool operator ==(const DestinationID &destination) const { return this->value == destination.value; }
// 	constexpr bool operator ==(const StationID &station) const { return this->value == station; }
// };

/**
 * Maximum number of orders in implicit-only lists before we start searching
 * harder for duplicates.
 */
pub const IMPLICIT_ORDER_ONLY_CAP: u32 = 32;

/** Order types. It needs to be 8bits, because we save and load it as such */
pub enum OrderType {
    Nothing = 0,
    GotoStation = 1,
    GotoDepot = 2,
    Loading = 3,
    LeaveStation = 4,
    Dummy = 5,
    GotoWaypoint = 6,
    Conditional = 7,
    Implicit = 8,
}

/**
 * Unloading order types.
 */
pub enum OrderUnloadType {
    /// Unload all cargo that the station accepts.
    UnloadIfPossible = 0,
    /// Force unloading all cargo onto the platform, possibly not getting paid.
    Unload = 1,
    /// Transfer all cargo onto the platform.
    Transfer = 2,
    /// Totally no unloading will be done.
    NoUnload = 4,
}

/**
 * Loading order types.
 */
pub enum OrderLoadType {
    /// Load as long as there is cargo that fits in the train.
    LoadIfPossible = 0,
    /// Full load all cargoes of the consist.
    FullLoad = 2,
    /// Full load a single cargo of the consist.
    FullLoadAny = 3,
    /// Do not load anything.
    NoLoad = 4,
}

/**
 * Non-stop order flags.
 */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = OrderNonStopFlags)]
pub enum OrderNonStopFlag {
    /// The vehicle will not stop at any stations it passes except the destination, aka non-stop.
    NonStop = 0,
    /// The vehicle will stop at any station it passes except the destination, aka via.
    GoVia = 1,
}

/**
 * Where to stop the trains.
 */
pub enum OrderStopLocation {
    /// Stop at the near end of the platform
    NearEnd = 0,
    /// Stop at the middle of the platform
    Middle = 1,
    /// Stop at the far end of the platform
    FarEnd = 2,
}

/**
 * Reasons that could cause us to go to the depot.
 */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = OrderDepotTypeFlags)]
pub enum OrderDepotTypeFlag {
    /// This depot order is because of the servicing limit.
    Service = 0,
    /// This depot order is because of a regular order.
    PartOfOrders = 1,
}

/**
 * Actions that can be performed when the vehicle enters the depot.
 */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = OrderDepotActionFlags)]
pub enum OrderDepotActionFlag {
    /// Service the vehicle and then halt it.
    Halt = 0,
    /// Send the vehicle to the nearest depot.
    NearestDepot = 1,
    /// Service the vehicle and then unbunch it.
    Unbunch = 2,
}

/**
 * Variables (of a vehicle) to 'cause' skipping on.
 */
pub enum OrderConditionVariable {
    /// Skip based on the amount of load
    LoadPercentage = 0,
    /// Skip based on the reliability
    Reliability = 1,
    /// Skip based on the maximum speed
    MaxSpeed = 2,
    /// Skip based on the age
    Age = 3,
    /// Skip when the vehicle requires service
    RequiresService = 4,
    /// Always skip
    Unconditionally = 5,
    /// Skip based on the remaining lifetime
    RemainingLifetime = 6,
    /// Skip based on the maximum reliability
    MaxReliability = 7,
    /// Skip when the train is driving backwards
    DrivingBackwards = 8,
}

/**
 * Comparator for the skip reasoning.
 */
pub enum OrderConditionComparator {
    /// Skip if both values are equal
    Equal = 0,
    /// Skip if both values are not equal
    NotEqual = 1,
    /// Skip if the value is less than the limit
    LessThan = 2,
    /// Skip if the value is less or equal to the limit
    LessThanOrEqual = 3,
    /// Skip if the value is more than the limit
    MoreThan = 4,
    /// Skip if the value is more or equal to the limit
    MoreThanOrEqual = 5,
    /// Skip if the variable is true
    IsTrue = 6,
    /// Skip if the variable is false
    IsFalse = 7,
}

/**
 * Enumeration for the data to set in #CmdModifyOrder.
 */
pub enum ModifyOrderFlags {
    /// Passes an OrderNonStopFlags.
    NewStop,
    /// Passes an OrderStopLocation.
    StopLocation,
    /// Passes an OrderUnloadType.
    Unload,
    /// Passes an OrderLoadType
    Load,
    /// Selects the OrderDepotAction
    DepotAction,
    /// A conditional variable changes.
    CondVariable,
    /// A comparator changes.
    CondComparator,
    /// The value to set the condition to.
    CondValue,
    /// Change the destination of a conditional order.
    CondDestination,
}

/**
 * Depot action to switch to when doing a #MOF_DEPOT_ACTION.
 */
pub enum OrderDepotAction {
    /// Always go to the depot
    AlwaysGo = 0,
    /// Service only if needed
    Service = 1,
    /// Go to the depot and stop there
    Stop = 2,
    /// Go to the depot and unbunch
    Unbunch = 3,
}

/**
 * Enumeration for the data to set in #CmdChangeTimetable.
 */
pub enum ModifyTimetableFlags {
    /// Set wait time.
    WaitTime,
    /// Set travel time.
    TravelTime,
    /// Set max travel speed.
    TravelSpeed,
}

/** Clone actions. */
pub enum CloneOptions {
    Share = 0,
    Copy = 1,
    Unshare = 2,
}
