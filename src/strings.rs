/*
 * This file is part of OpenTTD.
 * OpenTTD is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, version 2.
 * OpenTTD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details. You should have received a copy of the GNU General Public License along with OpenTTD. If not, see <https://www.gnu.org/licenses/old-licenses/gpl-2.0>.
 */

/** @file strings_type.h Types related to strings. */

/**
 * Numeric value that represents a string, independent of the selected language.
 */
pub struct StringId(pub u32);

impl StringId {
    /// Constant representing an invalid string (16bit in case it is used in savegames)
    pub const INVALID_STRING_ID: StringId = StringId(0xFFFF);
}
/// Max. length of UTF-8 encoded unicode character
pub const MAX_CHAR_LENGTH: i32 = 4;
/// Maximum number of languages supported by the game, and the NewGRF specs
pub const MAX_LANG: u32 = 0x7F;

/** Directions a text can go to */
pub enum TextDirection {
    /// Text is written left-to-right by default
    LTR,
    /// Text is written right-to-left by default
    RTL,
}

/** StringTabs to group StringIDs */
pub enum StringTab {
    /* Tabs 0..1 for regular strings */
    TOWN = 4,
    INDUSTRY = 9,
    STATION = 12,
    SPECIAL = 14,
    OLD_CUSTOM = 15,
    VEHICLE = 16,
    /* Tab 17 for regular strings */
    OLD_NEWGRF = 26,
    /// Start of GameScript supplied strings.
    GAMESCRIPT_START = 32,
    /// Start of NewGRF supplied strings.
    NEWGRF_START = 64,
}

/** The index/offset of a string within a #StringTab. */
pub struct StringIndexInTab(pub u32);

/** Number of bits for the StringIndex within a StringTab */
pub const TAB_SIZE_BITS: u32 = 11;
/** Number of strings per StringTab */
pub const TAB_SIZE: u32 = 1 << TAB_SIZE_BITS;

/** Number of strings for GameScripts */
pub const TAB_SIZE_GAMESCRIPT: u32 = TAB_SIZE * 32;

/** Number of strings for NewGRFs */
pub const TAB_SIZE_NEWGRF: u32 = TAB_SIZE * 256;

/** The number of builtin generators for town names. */
pub const BUILTIN_TOWNNAME_GENERATOR_COUNT: u32 = 21;

/** Special strings for town names. The town name is generated dynamically on request. */
pub const SPECSTR_TOWNNAME_START: StringId = StringId(0x20C0);
pub const SPECSTR_TOWNNAME_END: StringId =
    StringId(SPECSTR_TOWNNAME_START.0 + BUILTIN_TOWNNAME_GENERATOR_COUNT);

/** Special strings for company names on the form "TownName transport". */
pub const SPECSTR_COMPANY_NAME_START: StringId = StringId(0x70EA);
pub const SPECSTR_COMPANY_NAME_END: StringId =
    StringId(SPECSTR_COMPANY_NAME_START.0 + BUILTIN_TOWNNAME_GENERATOR_COUNT);

/// Special string for silly company names.
pub const SPECSTR_SILLY_NAME: StringId = StringId(0x70E5);
/// Special string for Surname & Co company names.
pub const SPECSTR_ANDCO_NAME: StringId = StringId(0x70E6);
/// Special string for the president's name.
pub const SPECSTR_PRESIDENT_NAME: StringId = StringId(0x70E7);

pub enum StringParameterData {
    None,
    U64(u64),
    String(String),
}

/** The data required to format and validate a single parameter of a string. */
pub struct StringParameter {
    /// The data of the parameter.
    pub data: StringParameterData,
    /// The #StringControlCode to interpret this data with when it's the first parameter, otherwise '\0'.
    pub ty: char,
}

/**
 * Container for an encoded string, created by GetEncodedString.
 */
#[derive(Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct EncodedString {
    /// The encoded string.
    string: String,
}

impl EncodedString {
    pub fn get_decoded_string(&self) -> String {}

    pub fn replace_param(&self, param: usize, value: &StringParameter) -> EncodedString {}

    pub fn clear(&mut self) {}

    pub fn empty(&self) -> bool {}

    pub fn get_encoded_string_with_args(
        str: StringId,
        params: &[&StringParameter],
    ) -> EncodedString {
    }
}
