use enum_bitset::EnumBitset;

/** @file vehicle_type.h Types related to vehicles. */

/** The type all our vehicle IDs have. */
pub type VehicleId = u32;
// using VehicleID = PoolID<uint32_t, struct VehicleIDTag, 0xFF000, 0xFFFFF>;

///< Acceleration due to gravity, 9.8 m/s^2
pub const GROUND_ACCELERATION: i32 = 9800;

/** Available vehicle types. It needs to be 8bits, because we save and load it as such */
pub enum VehicleType {
    /// %Train vehicle type.
    Train,
    /// Road vehicle type.
    Road,
    /// %Ship vehicle type.
    Ship,
    /// %Aircraft vehicle type.
    Aircraft,

    /// Effect vehicle type (smoke, explosions, sparks, bubbles)
    Effect,
    /// Disaster vehicle type.
    Disaster,

    /// Non-existing type of vehicle.
    Invalid = 0xFF,
}
// DECLARE_INCREMENT_DECREMENT_OPERATORS(VehicleType)
// DECLARE_ENUM_AS_ADDABLE(VehicleType)

impl VehicleType {
    /// Company-ownable type.
    pub const COMPANY: [VehicleType; 4] = [
        VehicleType::Train,
        VehicleType::Road,
        VehicleType::Ship,
        VehicleType::Aircraft,
    ];

    pub const ALL: [VehicleType; 6] = [
        VehicleType::Train,
        VehicleType::Road,
        VehicleType::Ship,
        VehicleType::Aircraft,
        VehicleType::Effect,
        VehicleType::Disaster,
    ];
}

/** Flags for goto depot commands. */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = DepotCommandFlags)]
pub enum DepotCommandFlag {
    ///< The vehicle will leave the depot right after arrival (service only)
    Service,
    ///< Tells that it's a mass send to depot command (type in VLW flag)
    MassSend,
    ///< Don't cancel current goto depot command if any
    DontCancel,
}

///< The maximum length of a vehicle name in characters including '\0'
pub const MAX_LENGTH_VEHICLE_NAME_CHARS: u32 = 32;

/** The length of a vehicle in tile units. */
pub const VEHICLE_LENGTH: u32 = 8;

/** Vehicle acceleration models. */
pub enum AccelerationModel {
    Original,
    Realistic,
}

/** Visualisation contexts of vehicles and engines. */
pub enum EngineImageType {
    /// Vehicle drawn in viewport.
    OnMap = 0x00,
    /// Vehicle drawn in depot.
    InDepot = 0x10,
    /// Vehicle drawn in vehicle details, refit window, ...
    InDetails = 0x11,
    /// Vehicle drawn in vehicle list, group list, ...
    InList = 0x12,
    /// Vehicle drawn in purchase list, autoreplace gui, ...
    Purchase = 0x20,
    /// Vehicle drawn in preview window, news, ...
    Preview = 0x21,
}

/** Randomisation triggers for vehicles */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = VehicleRandomTriggers)]
pub enum VehicleRandomTrigger {
    /// Affected vehicle only: Vehicle is loaded with cargo, after it was empty.
    NewCargo,
    /// Front vehicle only: Consist arrived in depot.
    Depot,
    /// Front vehicle only: Entire consist is empty.
    Empty,
    /// All vehicles in consist: Any vehicle in the consist received new cargo.
    AnyNewCargo,
    /// All vehicles in consist: 32 day callback requested rerandomisation
    Callback32,
}
