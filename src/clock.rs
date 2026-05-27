use crate::timer::TimerManager;
use std::hash::Hash;

pub trait Clock {
    type Period: Hash + Clone + Ord + PartialOrd + Eq + PartialEq;
	type Instant;
    type Elapsed;
    type Storage: Default;

    fn manager(&mut self) -> &mut TimerManager<Self>;

    /**
    	* Validate that a new period is actually valid.
    	*
    	* For most timers this is not an issue, but some want to make sure their
    	* period is unique, to ensure deterministic game-play.
    	*
    	* This is meant purely to protect a developer from making a mistake.
    	* As such, assert() when validation fails.
    	*
    	* @param period The period to validate.
    	*/
    fn assert(&self, period: Self::Period);

    /**
    	* Called when time for this timer elapsed.
    	*
    	* The implementation per type is different, but they all share a similar goal:
    	*   Call the Elapsed() method of all active timers.
    	*
    	* @param value The amount of time that has elapsed.
    	* @return True iff time has progressed.
    	*/
    fn elapse(&mut self, delta: Self::Elapsed) -> bool;
}
