use crate::cargo::{CargoLabel, CargoType, CargoTypes, MixedCargoType};
use crate::economy::Price;
use crate::landscape::LandscapeTypes;
use crate::newgrf::types::VehicleCallbackMasks;
use crate::rail::RailTypes;
use crate::road::RoadTypes;
use crate::sound::SoundId;
use crate::strings::StringId;
use enum_bitset::EnumBitset;
use crate::physics::Acceleration;
use crate::timer::{Calendar, Year};

/** Unique identification number of an engine. */
pub type EngineId = u16;

/** Available types of rail vehicles. */
#[derive(Default, Debug)]
pub enum RailVehicleType {
    ///< indicates a "standalone" locomotive
    Singlehead,
    ///< indicates a combination of two locomotives
    Multihead,
    ///< simple wagon, not motorized
    #[default]
    Wagon,
}

/** Type of rail engine. */
#[derive(Default, Debug)]
pub enum EngineClass {
    ///< Steam rail engine.
    #[default]
    Steam,
    ///< Diesel rail engine.
    Diesel,
    ///< Electric rail engine.
    Electric,
    ///< Mono rail engine.
    Monorail,
    ///< Maglev engine.
    Maglev,
}

/** Acceleration model of a vehicle. */
pub enum VehicleAccelerationModel {
    ///< Default acceleration model.
    Normal,
    ///< Monorail acceleration model.
    Monorail,
    ///< Maglev acceleration model.
    Maglev,
}

/** Meaning of the various bits of the visual effect. */
#[derive(EnumBitset, Copy, Clone, Debug)]
#[bitset(name = VisualEffects)]
pub enum VisualEffect {
    /// Use default from engine class
    Default = 0,
    /// Steam plumes
    Steam = 1,
    /// Diesel fumes
    Diesel = 2,
    /// Electric sparks
    Electric = 3,

    /// Flag to disable visual effect
    VE_DISABLE_EFFECT = 6,
    /// Flag to disable wagon power
    VE_DISABLE_WAGON_POWER = 7,

    /// Default value to indicate that visual effect should be based on engine class
    VE_DEFAULT = 0xFF,
}

impl Default for VisualEffect {
    fn default() -> Self {
        Self::VE_DEFAULT
    }
}

impl VisualEffect {
    /// First bit that contains the offset (0 = front, 8 = centre, 15 = rear)
    pub const VE_OFFSET_START: u8 = 0;
    /// Number of bits used for the offset
    pub const VE_OFFSET_COUNT: u8 = 4;
    /// Value of offset corresponding to a position above the centre of the vehicle
    pub const VE_OFFSET_CENTRE: u8 = 8;

    /// First bit used for the type of effect
    pub const VE_TYPE_START: u8 = 4;
    /// Number of bits used for the effect type
    pub const VE_TYPE_COUNT: u8 = 2;

    /// Flag for advanced effects
    pub const VE_ADVANCED_EFFECT: VisualEffect = VisualEffect::VE_DISABLE_EFFECT;
}

/** Information about a rail vehicle. */
#[derive(Default, Debug)]
pub struct RailVehicleInfo {
    pub image_index: u8,
    /// Type of rail vehicle.
    pub railveh_type: RailVehicleType,
    /// Purchase cost factor;      For multiheaded engines the sum of both engine prices.
    pub cost_factor: u8,
    ///< Railtypes, mangled if elrail is disabled.
    pub railtypes: RailTypes,
    /// Intended railtypes, regardless of elrail being enabled or disabled.
    pub intended_railtypes: RailTypes,
    /// Bit value to tell AI that this engine is for passenger use only
    pub ai_passenger_only: u8,
    /// Maximum speed (1 unit = 1/1.6 mph = 1 km-ish/h)
    pub max_speed: u16,
    /// Power of engine (hp);      For multiheaded engines the sum of both engine powers.
    pub power: u16,
    /// Weight of vehicle (tons);  For multiheaded engines the weight of each single engine.
    pub weight: u16,
    /// Running cost of engine;    For multiheaded engines the sum of both running costs.
    pub running_cost: u8,
    pub running_cost_class: Price,
    /// Class of engine for this vehicle
    pub engclass: EngineClass,
    /// Cargo capacity of vehicle; For multiheaded engines the capacity of each single engine.
    pub capacity: u8,
    /// Extra power applied to consist if wagon should be powered
    pub pow_wag_power: u16,
    /// Extra weight applied to consist if wagon should be powered
    pub pow_wag_weight: u8,
    /// Bitstuffed NewGRF visual effect data
    pub visual_effect: VisualEffects,
    /// length on main map for this type is 8 - shorten_factor
    pub shorten_factor: u8,
    /// Tractive effort coefficient
    pub tractive_effort: u8,
    /// Coefficient of air drag
    pub air_drag: u8,
    /// Property 0x25: "User-defined bit mask" Used only for (very few) NewGRF vehicles
    pub user_def_data: u8,
    /// Modifier to maximum speed in curves (fixed-point binary with 8 fractional bits)
    pub curve_speed_mod: u16,
}

/** Information about a ship vehicle. */
#[derive(Default, Debug)]
struct ShipVehicleInfo {
    pub image_index: u8,
    pub cost_factor: u8,
    pub running_cost: u8,
    /// Acceleration (1 unit = 1/3.2 mph per tick = 0.5 km-ish/h per tick)
    pub acceleration: Acceleration,
    /// Maximum speed (1 unit = 1/3.2 mph = 0.5 km-ish/h)
    pub max_speed: u8,
    pub capacity: u16,
    pub sfx: SoundId,
    /// Is ship refittable; only used during initialisation. Later use EngineInfo::refit_mask.
    pub old_refittable: bool,
    /// Bitstuffed NewGRF visual effect data
    pub visual_effect: VisualEffects,
    /// Fraction of maximum speed for ocean tiles.
    pub ocean_speed_frac: u8,
    /// Fraction of maximum speed for canal/river tiles.
    pub canal_speed_frac: u8,
}

impl ShipVehicleInfo {
    pub fn speed_frac(&self, is_ocean: bool) -> u8 {
        if is_ocean {
            self.ocean_speed_frac
        } else {
            self.canal_speed_frac
        }
    }
    /**
    	* Apply ocean/canal speed fraction to a velocity.
    	* @param raw_speed The original speed.
    	* @param is_ocean Whether to apply the ocean or canal/river speed fraction.
    	* @return The actual maximum speed of the ship.
    	*/
    pub fn apply_water_class_speed_frac(&self, raw_speed: u32, is_ocean: bool) -> u32 {
        /* speed_frac == 0 means no reduction while 0xFF means reduction to 1/256. */
        raw_speed * (256 - self.speed_frac(is_ocean) as u32) / 256
    }
}

/**
 * AircraftVehicleInfo subtypes, bitmask type.
 * If bit 0 is 0 then it is a helicopter, otherwise it is a plane
 * in which case bit 1 tells us whether it's a big(fast) plane or not.
 */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = AircraftSubTypeBits)]
pub enum AircraftSubTypeBit {
    Heli,
    /// Conventional Take Off and Landing, i.e. planes
    Ctol,
    Fast,
}

/** Information about a aircraft vehicle. */
pub struct AircraftVehicleInfo {
    pub image_index: u8,
    pub cost_factor: u8,
    pub running_cost: u8,
    /// Type of aircraft. @see AircraftSubTypeBits
    pub subtype: AircraftSubTypeBits,
    pub sfx: SoundId,
    /// Maximum speed (1 unit = 8 mph = 12.8 km-ish/h)
    pub max_speed: u16,
    pub acceleration: Acceleration,
    /// Mail capacity (bags).
    pub mail_capacity: u8,
    /// Passenger capacity (persons).
    pub passenger_capacity: u16,
    /// Maximum range of this aircraft.
    pub max_range: u16,
}

/** Information about a road vehicle. */
pub struct RoadVehicleInfo {
    pub image_index: u8,
    pub cost_factor: u8,
    pub running_cost: u8,
    pub running_cost_class: Price,
    pub sfx: SoundId,
    /// Maximum speed (1 unit = 1/3.2 mph = 0.5 km-ish/h)
    pub max_speed: u16,
    pub capacity: u8,
    /// Weight in 1/4t units
    pub weight: u8,
    /// Power in 10hp units
    pub power: u8,
    /// Coefficient of tractive effort
    //TODO: Defaults to 0x4C
    pub tractive_effort: u8,
    /// Coefficient of air drag
    pub air_drag: u8,
    /// Bitstuffed NewGRF visual effect data
    pub visual_effect: VisualEffects,
    /// length on main map for this type is 8 - shorten_factor
    pub shorten_factor: u8,
    /// Road type
    pub roadtype: RoadTypes,
}

/**
 * Extra engine flags for NewGRF features.
 * This is defined in the specification a 32 bit value, but most bits are not currently used.
 */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = ExtraEngineFlags)]
pub enum ExtraEngineFlag {
    /// No 'new vehicle' news will be generated.
    NoNews,
    /// No exclusive preview will be offered.
    NoPreview,
    /// Engine will join exclusive preview with variant parent.
    JoinPreview,
    /// Engine reliability will be synced with variant parent.
    SyncReliability,
    /// Train wagon has a cab and can lead a train when backing up, without any speed reduction.
    HasCab,
}

/**
 * EngineInfo.misc_flags is a bitmask, with the following values
 */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = EngineMiscFlags)]
pub enum EngineMiscFlag {
    /// Rail vehicle tilts in curves
    RailTilts = 0,

    /// Vehicle uses two company colours
    Uses2CC = 1,

    /// Rail vehicle is a multiple-unit (DMU/EMU)
    RailIsMU = 2,
    /// Rail vehicle has old depot-flip handling
    RailFlips = 3,

    /// Automatic refitting is allowed
    AutoRefit = 4,
    /// Use the new capacity algorithm. The default cargotype of the vehicle does not affect capacity multipliers. CB 15 is also called in purchase list.
    NoDefaultCargoMultiplier = 5,
    /// Do not show black smoke during a breakdown.
    NoBreakdownSmoke = 6,
    /// Draw vehicle by stacking multiple sprites.
    SpriteStack = 7,
}

impl EngineMiscFlag {
    /// Road vehicle is a tram/light rail vehicle
    pub const RoadIsTram: EngineMiscFlag = EngineMiscFlag::RailTilts;
}

/**
 * Information about a vehicle
 *  @see table/engines.h
 */
struct EngineInfo {
    /// Basic date of engine introduction (without random parts).
    pub base_intro: Year<Calendar>,
    /// Lifetime of a single vehicle
    pub lifelength: Year<Calendar>,
    /// Basic duration of engine availability (without random parts). \c 0xFF means infinite life.
    pub base_life: Year<Calendar>,
    pub decay_speed: u8,
    pub load_amount: u8,
    /// Climates supported by the engine.
    pub climates: LandscapeTypes,
    pub cargo_type: CargoType,
    pub cargo_label: EngineCargo,
    pub refit_mask: CargoTypes,
    pub refit_cost: u8,
    /// Miscellaneous flags. @see EngineMiscFlags
    pub misc_flags: EngineMiscFlags,
    /// Bitmask of vehicle callbacks that have to be called
    pub callback_mask: VehicleCallbackMasks,
    /// Number of years early to retire vehicle
    pub retire_early: i8,
    pub extra_flags: ExtraEngineFlags,
    /// Default name of engine
    pub string_id: StringId,
    /// Number of ticks before carried cargo is aged.
    pub cargo_age_period: u16,
    /// Engine variant ID. If set, will be treated specially in purchase lists.
    pub variant_id: EngineId,
}

pub enum EngineCargo {
    Single(CargoLabel),
    Mixed(MixedCargoType),
}

/**
 * Engine.flags is a bitmask, with the following values.
 */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = EngineFlags)]
pub enum EngineFlag {
    /// This vehicle is available to everyone.
    Available,
    /// This vehicle is in the exclusive preview stage, either being used or being offered to a company.
    ExclusivePreview,
}

/**
 * Contexts an engine name can be shown in.
 */
pub enum EngineNameContext {
    /// No specific context available.
    Generic = 0x00,
    /// Name is shown in the vehicle details GUI.
    VehicleDetails = 0x11,
    /// Name is shown in the purchase list (including autoreplace window 'Available vehicles' panel).
    PurchaseList = 0x20,
    /// Name is shown in exclusive preview or newspaper.
    PreviewNews = 0x21,
    /// Name is show in the autoreplace window 'Vehicles in use' panel.
    AutoreplaceVehicleInUse = 0x22,
}

/**
 * Combine an engine ID and a name context to an engine name StringParameter.
 * @param engine_id The engine's ID.
 * @param context The context for the name.
 * @param extra_data Arbitrary extra data.
 * @return The packed StringParameter.
 */
pub fn pack_engine_name_dparam(
    engine_id: EngineId,
    context: EngineNameContext,
    extra_data: u32,
) -> u64 {
    (engine_id as u64) | ((context as u64) << 32) | ((extra_data as u64) << 40)
}

/// The maximum length of an engine name in characters including '\0'
pub const MAX_LENGTH_ENGINE_NAME_CHARS: u32 = 32;
