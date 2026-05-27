use crate::clock::Clock;
use crate::timer::manager::{TimerImpl, TimerKey};

/** @file timer.h Definition of Interval and OneShot timers. */

/**
 * An interval timer will fire every interval, and will continue to fire until it is deleted.
 *
 * The callback receives how many times the timer has fired since the last time it fired.
 * It will always try to fire every interval, but in times of severe stress it might be late.
 *
 * Each Timer-type needs to implement the Elapsed() method, and call the callback if needed.
 *
 * Setting the period to zero disables the interval. It can be reenabled at any time by
 * calling SetInterval() with a non-zero period.
 */
pub struct IntervalTimer<T: Clock>(TimerKey<T>);

impl<T: Clock> Drop for IntervalTimer<T> {
	fn drop(&mut self) {
		T::manager().unregister(&self.0);
	}
}

impl<T: Clock> IntervalTimer<T> {
	/**
	 * Create a new interval timer.
	 *
	 * @param interval The interval between each callback.
	 * @param callback The callback to call when the interval has passed.
	 */
	pub fn new(period: T::Period, callback: Box<dyn Fn(u32)>) -> Self {
		let key = TimerKey(period, callback);
		T::manager().register(
			key.clone(),
			TimerImpl::Interval {
				storage: T::Storage::default(),
			},
		);
		Self(key)
	}

	/**
	 * Set a new interval for the timer.
	 *
	 * @param interval The interval between each callback.
	 * @param reset Whether to reset the timer to zero.
	 */
	pub fn set_interval(&mut self, interval: T::Period, reset: bool) {
		T::manager().change_period(&mut self.0, interval, reset);
	}
}