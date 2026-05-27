use core::num::Saturating;
use std::mem;
use crate::station::StationId;

pub type Money = Saturating<u64>;

/** Type of the game economy. */
#[repr(u8)]
pub enum EconomyType {
    ORIGINAL = 0,
    SMOOTH = 1,
    FROZEN = 2,
}

/**
 * Minimum allowed value of town_cargo_scale/industry_cargo_scale.
 * Below 13, callback-based industries would produce less than once per month. We round up to 15% because it's a nicer number.
 * Towns use the same minimum to match, and because below this small towns often produce no cargo.
 */
pub const MIN_CARGO_SCALE: i32 = 15;
/**
 * Maximum allowed value of town_cargo_scale/industry_cargo_scale.
 * Above 340, callback-based industries would produce more than once per day, which GRFs do not expect.
 * Towns use the same maximum to match.
 */
pub const MAX_CARGO_SCALE: i32 = 300;
/** Default value of town_cargo_scale/industry_cargo_scale. */
pub const DEF_CARGO_SCALE: i32 = 100;

/** Data of the economy. */
pub struct Economy {
    /// NOSAVE: Maximum possible loan
    pub max_loan: Money,
    /// Economy fluctuation status
    pub fluct: i16,
    /// Interest
    pub interest_rate: u8,
    /// inflation amount
    pub infl_amount: u8,
    /// inflation rate for payment rates
    pub infl_amount_pr: u8,
    /// Bits 31-16 are number of industry to be performed, 15-0 are fractional collected daily
    pub industry_daily_change_counter: u32,
    /// The value which will increment industry_daily_change_counter. Computed value. NOSAVE
    pub industry_daily_increment: u32,
    /// Cumulated inflation of prices since game start; 16 bit fractional part
    pub inflation_prices: u64,
    /// Cumulated inflation of cargo payment since game start; 16 bit fractional part
    pub inflation_payment: u64,

    /* Old stuff for savegame conversion only */
    /// Old: Unrounded max loan
    pub old_max_loan_unround: Money,
    /// Old: Fraction of the unrounded max loan
    pub old_max_loan_unround_fract: u16,
}

/** Score categories in the detailed performance rating. */
#[repr(u8)]
pub enum ScoreID {
    Vehicles = 0,
    Stations = 1,
    MinProfit = 2,
    MinIncome = 3,
    MaxIncome = 4,
    Delivered = 5,
    Cargo = 6,
    Money = 7,
    Loan = 8,
    /// This must always be the last entry
    Total = 9,
}

/**
 * The max score that can be in the performance history.
 * The scores together of score_info is allowed to be more!
 */
pub const SCORE_MAX: i32 = 1000;

/** Data structure for storing how the score is computed for a single score id. */
pub struct ScoreInfo {
    /// How much you need to get the perfect score
    pub needed: i32,
    /// How much score it will give
    pub score: i32,
}

/**
 * Enumeration of all base prices for use with #Prices.
 * The prices are ordered as they are expected by NewGRF cost multipliers, so don't shuffle them.
 */
#[derive(Default, Debug)]
#[repr(u8)]
pub enum Price {
	#[default]
    StationValue = 0,
    BuildRail,
    BuildRoad,
    BuildSignals,
    BuildBridge,
    BuildDepotTrain,
    BuildDepotRoad,
    BuildDepotShip,
    BuildTunnel,
    BuildStationRail,
    BuildStationRailLength,
    BuildStationAirport,
    BuildStationBus,
    BuildStationTruck,
    BuildStationDock,
    BuildVehicleTrain,
    BuildVehicleWagon,
    BuildVehicleAircraft,
    BuildVehicleRoad,
    BuildVehicleShip,
    BuildTrees,
    Terraform,
    ClearGrass,
    ClearRough,
    ClearRocks,
    ClearFields,
    ClearTrees,
    ClearRail,
    ClearSignals,
    ClearBridge,
    ClearDepotTrain,
    ClearDepotRoad,
    ClearDepotShip,
    ClearTunnel,
    ClearWater,
    ClearStationRail,
    ClearStationAirport,
    ClearStationBus,
    ClearStationTruck,
    ClearStationDock,
    ClearHouse,
    ClearRoad,
    RunningTrainSteam,
    RunningTrainDiesel,
    RunningTrainElectric,
    RunningAircraft,
    RunningRoadveh,
    RunningShip,
    BuildIndustry,
    ClearIndustry,
    BuildObject,
    ClearObject,
    BuildWaypointRail,
    ClearWaypointRail,
    BuildWaypointBuoy,
    ClearWaypointBuoy,
    TownAction,
    BuildFoundation,
    BuildIndustryRaw,
    BuildTown,
    BuildCanal,
    ClearCanal,
    BuildAqueduct,
    ClearAqueduct,
    BuildLock,
    ClearLock,
    InfrastructureRail,
    InfrastructureRoad,
    InfrastructureWater,
    InfrastructureStation,
    InfrastructureAirport,
}

impl Price {
    pub const COUNT: usize = mem::variant_count::<Price>();
}

///< Prices of everything. See [`Price`].
pub type Prices = [Money; Price::COUNT];
pub type PriceMultipliers = [i8; Price::COUNT];

/** Types of expenses. */
#[repr(u8)]
pub enum ExpensesType {
    ///< Construction costs.
    Construction = 0,
    ///< New vehicles.
    NewVehicles,
    ///< Running costs trains.
    TrainRun,
    ///< Running costs road vehicles.
    RoadvehRun,
    ///< Running costs aircraft.
    AircraftRun,
    ///< Running costs ships.
    ShipRun,
    ///< Property costs.
    Property,
    ///< Revenue from trains.
    TrainRevenue,
    ///< Revenue from road vehicles.
    RoadvehRevenue,
    ///< Revenue from aircraft.
    AircraftRevenue,
    ///< Revenue from ships.
    ShipRevenue,
    ///< Interest payments over the loan.
    LoanInterest,
    ///< Other expenses.
    Other,
    ///< Number of expense types.
    END,
    ///< Invalid expense type.
    InvalidExpenses = 0xFF,
}

/**
 * Data type for storage of Money for each #ExpensesType category.
 */
pub type Expenses = [Money; ExpensesType::END as usize];

/**
 * Categories of a price bases.
 */
#[repr(u8)]
pub enum PriceCategory {
    ///< Not affected by difficulty settings
    NONE,
    ///< Price is affected by "vehicle running cost" difficulty setting
    RUNNING,
    ///< Price is affected by "construction cost" difficulty setting
    CONSTRUCTION,
}

/** The "steps" in loan size, in British Pounds! */
pub const LOAN_INTERVAL: i32 = 10000;
/** The size of loan for a new company, in British Pounds! */
pub const INITIAL_LOAN: i64 = 100000;
/** The max amount possible to configure for a max loan of a company. */
pub const MAX_LOAN_LIMIT: i64 = 2000000000;

/**
 * Maximum inflation (including fractional part) without causing overflows in int64_t price computations.
 * This allows for 32 bit base prices (21 are currently needed).
 * Considering the sign bit and 16 fractional bits, there are 15 bits left.
 * 170 years of 4% inflation result in a inflation of about 822, so 10 bits are actually enough.
 * Note that NewGRF multipliers share the 16 fractional bits.
 * @see MAX_PRICE_MODIFIER
 */
pub const MAX_INFLATION: u64 = (1u64 << (63 - 32)) - 1;

/**
 * Maximum NewGRF price modifiers.
 * Increasing base prices by factor 65536 should be enough.
 * @see MAX_INFLATION
 */
pub const MIN_PRICE_MODIFIER: i32 = -8;
pub const MAX_PRICE_MODIFIER: i32 = 16;
pub const INVALID_PRICE_MODIFIER: i32 = MIN_PRICE_MODIFIER - 1;

/** Multiplier for how many regular track bits a tunnel/bridge counts. */
pub const TUNNELBRIDGE_TRACKBIT_FACTOR: u32 = 4;
/** Multiplier for how many regular track bits a level crossing counts. */
pub const LEVELCROSSING_TRACKBIT_FACTOR: u32 = 2;
/** Multiplier for how many regular track bits a road depot counts. */
pub const ROAD_DEPOT_TRACKBIT_FACTOR: u32 = 2;
/** Multiplier for how many regular track bits a bay stop counts. */
pub const ROAD_STOP_TRACKBIT_FACTOR: u32 = 2;
/** Multiplier for how many regular tiles a lock counts. */
pub const LOCK_DEPOT_TILE_FACTOR: u32 = 2;

pub struct CargoPayment {
	/// The current station
	pub station: StationId,
	///< The amount of money to add/remove from the bank account
	pub route_profit: Money,
	///< The visual profit to show
	pub visual_profit: Money,
	///< The transfer credits to be shown
	pub visual_transfer: Money,
}

pub struct CargoPaymentID(pub u32);

impl CargoPaymentID {
    pub const END: CargoPaymentID = CargoPaymentID(0xFF000);
    pub const INVALID: CargoPaymentID = CargoPaymentID(0xFFFFF);
}
