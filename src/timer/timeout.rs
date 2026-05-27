use crate::clock::Clock;

/**
 * A timeout timer will fire once after the interval. You can reset it to fire again.
 * The timer will never fire before the interval has passed, but in times of severe stress it might be late.
 */
pub struct TimeoutTimer<T: Clock> {
	///< The period of the timer.
	period: T::Period,
	///< The storage of the timer.
	storage: T::Storage,
	///< Whether the timeout has occurred.
	has_fired: bool,
	callback: Box<dyn Fn(u32)>,
}

impl<T: Clock> Drop for TimeoutTimer<T> {
	fn drop(&mut self) {
		T::manager().unregister(&self.0);
	}
}

impl<T: Clock> TimeoutTimer<T> {
	/**
	 * Create a new timeout timer.
	 *
	 * By default the timeout starts aborted; you will have to call Reset() before it starts.
	 *
	 * @param timeout The timeout after which the timer will fire.
	 * @param callback The callback to call when the timeout has passed.
	 * @param start Whether to start the timer immediately. If false, you can call Reset() to start it.
	 */
	pub fn new(timeout: T::Period, start: bool, callback: Box<dyn Fn(u32)>) -> Box<Self> {
		let timer = Box::new(Self {
			period,
			storage: T::Storage::default(),
			has_fired: !start,
			callback,
		});
		T::register(&timer);
		timer
	}

	/**
	 * Reset the timer, so it will fire again after the timeout.
	 */
	pub fn reset(&mut self) {
		self.has_fired = false;
		self.storage = T::Storage::default();
	}

	/**
	 * Reset the timer, so it will fire again after the timeout.
	 *
	 * @param timeout Set a new timeout for the next trigger.
	 */
	pub fn reset_to(&mut self, timeout: T::Period) {
		// TimerManager<TTimerType>::ChangeRegisteredTimerPeriod(*this, timeout);
		self.reset();
	}

	/**
	 * Abort the timer so it doesn't fire if it hasn't yet.
	 */
	pub fn abort(&mut self) {
		self.has_fired = true;
	}

	/**
	 * Check whether the timeout occurred.
	 *
	 * @return True iff the timeout occurred.
	 */
	pub fn has_fired(&self) -> bool {
		self.has_fired
	}
}
