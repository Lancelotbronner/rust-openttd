use crate::timer::{TickCounter, Ticks};
use enum_bitset::EnumBitset;
use crate::order::VehicleOrderId;

/** Bit numbers in #Vehicle::vehicle_flags. */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = VehicleFlags)]
pub enum VehicleFlag {
	/// Vehicle has finished loading.
	LoadingFinished,
	/// Vehicle is unloading cargo.
	CargoUnloading,
	/// Vehicle is a prototype (accepted as exclusive preview).
	BuiltAsPrototype,
	/// Whether the vehicle has started running on the timetable yet.
	TimetableStarted,
	/// Whether the vehicle should fill in the timetable automatically.
	AutofillTimetable,
	/// Whether non-destructive autofill should preserve waiting times
	AutofillPreserveWaitTime,
	/// Don't load anymore during the next load cycle.
	StopLoading,
	/// Vehicle's pathfinder is lost.
	PathfinderLost,
	/// Service interval is custom.
	ServiceIntervalIsCustom,
	/// Service interval is percent.
	ServiceIntervalIsPercent,
	/// Vehicle is driving backwards.
	DrivingBackwards,
}

/** Various front vehicle properties that are preserved when autoreplacing, using order-backup or switching front engines within a consist. */
pub struct Concist {
	/// Name of vehicle
	name: String,
	/* Used for timetabling. */
	/// How many ticks have passed since this order started.
	pub current_order_time: Ticks,
	/// How many ticks late (or early if negative) this vehicle is.
	pub lateness_counter: Ticks,
	/// At what tick of TimerGameTick::counter the vehicle should start its timetable.
	pub timetable_start: TickCounter,

	/// When the vehicle last left its unbunching depot.
	pub depot_unbunching_last_departure: TickCounter,
	/// When the vehicle will next try to leave its unbunching depot.
	pub depot_unbunching_next_departure: TickCounter,
	/// How many ticks for a single circumnavigation of the orders.
	pub round_trip_time: Ticks,

	/// The interval for (automatic) servicing; either in days or %.
	pub service_interval: u16,

	/// The index to the current real (non-implicit) order
	pub cur_real_order_index: VehicleOrderId,
	/// The index to the current implicit order
	pub cur_implicit_order_index: VehicleOrderId,

	/// Used for gradual loading and other miscellaneous things (@see VehicleFlags enum)
	pub vehicle_flags: VehicleFlags,
}

impl Concist {
	pub fn CopyConsistPropertiesFrom(src: &Concist) {}

	pub fn ResetDepotUnbunching() {}
}