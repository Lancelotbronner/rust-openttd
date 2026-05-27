use enum_bitset::EnumBitset;

pub struct RailTypeLabel(pub u32);

impl RailTypeLabel {
    pub const fn new(bytes: &[u8; 4]) -> Self {
        Self(u32::from_ne_bytes(*bytes))
    }

    pub const RAIL: RailTypeLabel = RailTypeLabel::new(b"RAIL");
    pub const ELECTRIC: RailTypeLabel = RailTypeLabel::new(b"ELRL");
    pub const MONO: RailTypeLabel = RailTypeLabel::new(b"MONO");
    pub const MAGLEV: RailTypeLabel = RailTypeLabel::new(b"MGLV");
}

/**
 * Enumeration for all possible railtypes.
 */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = RailTypes)]
pub enum RailType {
    /// Standard non-electric rails
    Rail,
    /// Electric rails
    Electric,
    /// Monorail
    Mono,
    /// Maglev
    Maglev,
    /// Flag for invalid railtype
    Invalid = 0xFF,
}
//TODO: DECLARE_INCREMENT_DECREMENT_OPERATORS(RailType)
