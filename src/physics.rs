/// Maximum speed (1 unit = 1/1.6 mph = 1 km-ish/h)
#[derive(Default, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct MaxSpeed<T>(pub T);

/// Power of engine (hp)
#[derive(Default, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Power<T>(pub T);

/// Acceleration (1 unit = 1/3.2 mph per tick = 0.5 km-ish/h per tick)
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Acceleration(pub u8);

impl Default for Acceleration {
	fn default() -> Self {
		Self(1)
	}
}