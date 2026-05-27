/*
 * This file is part of OpenTTD.
 * OpenTTD is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, version 2.
 * OpenTTD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details. You should have received a copy of the GNU General Public License along with OpenTTD. If not, see <https://www.gnu.org/licenses/old-licenses/gpl-2.0>.
 */

/** @file sound_type.h Types related to sounds. */
use std::fs::File;
use std::rc::Rc;


/** Source of loaded sound data. */
#[derive(Debug, Copy, Clone)]
pub enum SoundSource {
    /// The TTD format with certain internal bugs (incorrect frequency, WAV without headers, ...).
    BasesetOldFormat,
    /// New format with Opus and WAV files.
    BasesetNewFormat,
    /// Contained within a NewGRF.
    NewGRF,
}

#[derive(Debug)]
pub struct SoundEntry {
    pub data: Rc<Vec<u8>>,
    pub file: File,
    pub file_offset: usize,
    pub file_size: usize,
    pub rate: u16,
    pub bits_per_sample: u8,
    pub channels: u8,
    pub volume: u8,
    pub priority: u8,
    pub source: SoundSource,
    /// NewGRF container version if the sound is from a NewGRF.
    pub grf_container_ver: u8,
}

/**
 * Sound effects from baseset.
 *
 * This enum contains the sound effects from the sound baseset.
 * For hysterical raisins the order of sound effects in the baseset
 * is different to the order they are referenced in TTD/NewGRF.
 *  - The first two sound effects from the baseset are inserted at position 39.
 *    (see translation table _sound_idx)
 *  - The order in the enum is the order using in TTD/NewGRF.
 *  - The naming of the enum values includes the position in the baseset.
 * That is, for sound effects 0x02 to 0x28 the naming is off-by-two.
 */
pub enum SoundFx {
    ///  0 == 0x00  Construction: water infrastructure
    SND_02_CONSTRUCTION_WATER,
    ///  1 == 0x01  Industry producing: factory: whistle
    SND_03_FACTORY,
    ///  2 == 0x02  Station departure: steam engine
    SND_04_DEPARTURE_STEAM,
    ///  3 == 0x03  Train enters tunnel: steam engine
    SND_05_TRAIN_THROUGH_TUNNEL,
    ///  4 == 0x04  Station departure: cargo ships
    SND_06_DEPARTURE_CARGO_SHIP,
    ///  5 == 0x05  Station departure: passenger ships
    SND_07_DEPARTURE_FERRY,
    ///  6 == 0x06  Takeoff: propeller plane (non-toyland)
    SND_08_TAKEOFF_PROPELLER,
    ///  7 == 0x07  Takeoff: regular jet plane
    SND_09_TAKEOFF_JET,
    ///  8 == 0x08  Station departure: diesel and electric engine
    SND_0A_DEPARTURE_TRAIN,
    ///  9 == 0x09  Industry animation: coal/copper/gold mine: headgear
    SND_0B_MINE,
    /// 10 == 0x0A  Industry animation: power station: spark
    SND_0C_POWER_STATION,
    /// 11 == 0x0B  unused (1)
    SND_0D_UNUSED,
    /// 12 == 0x0C  Train passes through level crossing
    SND_0E_LEVEL_CROSSING,
    /// 13 == 0x0D  Breakdown: road vehicle (non-toyland)
    SND_0F_BREAKDOWN_ROADVEHICLE,
    /// 14 == 0x0E  Breakdown: train or ship (non-toyland)
    SND_10_BREAKDOWN_TRAIN_SHIP,
    /// 15 == 0x0F  unused (2)
    SND_11_UNUSED,
    /// 16 == 0x10  Destruction, crashes, disasters, ...
    SND_12_EXPLOSION,
    /// 15 == 0x11  Train+train crash
    SND_13_TRAIN_COLLISION,
    /// 18 == 0x12  Income from cargo delivery
    SND_14_CASHTILL,
    /// 19 == 0x13  GUI button click
    SND_15_BEEP,
    /// 20 == 0x14  News ticker
    SND_16_NEWS_TICKER,
    /// 21 == 0x15  Plane landing / touching ground
    SND_17_SKID_PLANE,
    /// 22 == 0x16  Takeoff: helicopter
    SND_18_TAKEOFF_HELICOPTER,
    /// 23 == 0x17  Station departure: truck and old bus (1) (non-toyland)
    SND_19_DEPARTURE_OLD_RV_1,
    /// 24 == 0x18  Station departure: truck and old bus (2) (random variation of SND_19_DEPARTURE_OLD_RV_1) (non-toyland)
    SND_1A_DEPARTURE_OLD_RV_2,
    /// 25 == 0x19  Station departure: modern bus (non-toyland)
    SND_1B_DEPARTURE_MODERN_BUS,
    /// 26 == 0x1A  Station departure: old bus (non-toyland)
    SND_1C_DEPARTURE_OLD_BUS,
    /// 27 == 0x1B  News: first vehicle at station
    SND_1D_APPLAUSE,
    /// 28 == 0x1C  News: new engine available
    SND_1E_NEW_ENGINE,
    /// 29 == 0x1D  Construction: other (non-water, non-rail, non-bridge)
    SND_1F_CONSTRUCTION_OTHER,
    /// 30 == 0x1E  Construction: rail infrastructure
    SND_20_CONSTRUCTION_RAIL,
    /// 31 == 0x1F  Road reconstruction animation
    SND_21_ROAD_WORKS,
    /// 32 == 0x20  unused (3)
    SND_22_UNUSED,
    /// 33 == 0x21  unused (4)
    SND_23_UNUSED,
    /// 34 == 0x22  Industry producing: farm (1): sheep
    SND_24_FARM_1,
    /// 35 == 0x23  Industry producing: farm (2): cow
    SND_25_FARM_2,
    /// 36 == 0x24  Industry producing: farm (3): horse
    SND_26_FARM_3,
    /// 37 == 0x25  Construction: bridge
    SND_27_CONSTRUCTION_BRIDGE,
    /// 38 == 0x26  Industry producing: sawmill
    SND_28_SAWMILL,
    /// 39 == 0x27  New year: performance improved
    SND_00_GOOD_YEAR,
    /// 40 == 0x28  New year: performance declined
    SND_01_BAD_YEAR,
    /// 41 == 0x29  Industry animation: sugar mine (2): shaking sieve
    SND_29_SUGAR_MINE_2,
    /// 42 == 0x2A  Industry animation: toy factory (3): eject product
    SND_2A_TOY_FACTORY_3,
    /// 43 == 0x2B  Industry animation: toy factory (2): stamp product
    SND_2B_TOY_FACTORY_2,
    /// 44 == 0x2C  Industry animation: toy factory (1): conveyor belt
    SND_2C_TOY_FACTORY_1,
    /// 45 == 0x2D  Industry animation: sugar mine (1): shaking sieve
    SND_2D_SUGAR_MINE_1,
    /// 46 == 0x2E  Industry animation: bubble generator (1): generate bubble
    SND_2E_BUBBLE_GENERATOR,
    /// 47 == 0x2F  Industry animation: bubble generator (2a): bubble pop
    SND_2F_BUBBLE_GENERATOR_FAIL,
    /// 48 == 0x30  Industry animation: toffee quarry: drill
    SND_30_TOFFEE_QUARRY,
    /// 49 == 0x31  Industry animation: bubble generator (2b): bubble slurped
    SND_31_BUBBLE_GENERATOR_SUCCESS,
    /// 50 == 0x32  unused (5)
    SND_32_UNUSED,
    /// 51 == 0x33  Industry producing: plastic fountain
    SND_33_PLASTIC_MINE,
    /// 52 == 0x34  Tree ambient: arctic snow (1): wind
    SND_34_ARCTIC_SNOW_1,
    /// 53 == 0x35  Breakdown: road vehicle (toyland)
    SND_35_BREAKDOWN_ROADVEHICLE_TOYLAND,
    /// 54 == 0x36  Industry animation: lumber mill (3): crashing tree
    SND_36_LUMBER_MILL_3,
    /// 55 == 0x37  Industry animation: lumber mill (2): falling tree
    SND_37_LUMBER_MILL_2,
    /// 56 == 0x38  Industry animation: lumber mill (1): chainsaw
    SND_38_LUMBER_MILL_1,
    /// 57 == 0x39  Tree ambient: arctic snow (2): heavy wind
    SND_39_ARCTIC_SNOW_2,
    /// 58 == 0x3A  Breakdown: train or ship (toyland)
    SND_3A_BREAKDOWN_TRAIN_SHIP_TOYLAND,
    /// 59 == 0x3B  Takeoff: supersonic plane (fast)
    SND_3B_TAKEOFF_JET_FAST,
    /// 60 == 0x3C  Station departure: bus (1) (toyland)
    SND_3C_DEPARTURE_BUS_TOYLAND_1,
    /// 61 == 0x3D  Takeoff: huge jet plane (high capacity)
    SND_3D_TAKEOFF_JET_BIG,
    /// 62 == 0x3E  Station departure: bus (2) (toyland)
    SND_3E_DEPARTURE_BUS_TOYLAND_2,
    /// 63 == 0x3F  Station departure: truck (1) (toyland)
    SND_3F_DEPARTURE_TRUCK_TOYLAND_1,
    /// 64 == 0x40  Station departure: truck (2) (toyland)
    SND_40_DEPARTURE_TRUCK_TOYLAND_2,
    /// 65 == 0x41  Station departure: maglev engine
    SND_41_DEPARTURE_MAGLEV,
    /// 66 == 0x42  Tree ambient: rainforest ambient (1): bird (1)
    SND_42_RAINFOREST_1,
    /// 67 == 0x43  Tree ambient: rainforest ambient (2): lion
    SND_43_RAINFOREST_2,
    /// 68 == 0x44  Tree ambient: rainforest ambient (3): monkeys
    SND_44_RAINFOREST_3,
    /// 69 == 0x45  Takeoff: propeller plane (1) (toyland)
    SND_45_TAKEOFF_PROPELLER_TOYLAND_1,
    /// 70 == 0x46  Takeoff: propeller plane (2) (toyland)
    SND_46_TAKEOFF_PROPELLER_TOYLAND_2,
    /// 71 == 0x47  Station departure: monorail engine
    SND_47_DEPARTURE_MONORAIL,
    /// 72 == 0x48  Tree ambient: rainforest ambient (4): bird (2)
    SND_48_RAINFOREST_4,
}

/** The number of sounds in the original sample.cat */
pub const ORIGINAL_SAMPLE_COUNT: u32 = 73;

#[derive(Default, Debug)]
pub struct SoundId(pub u16);

impl SoundId {
    pub const INVALID: SoundId = SoundId(0xFFFF);
}

pub const SOUND_EFFECT_MAX_VOLUME: u8 = 128;
