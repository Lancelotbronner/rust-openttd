use crate::clock::Clock;
use crate::gfx::MILLISECONDS_PER_TICK;
use crate::manager::{TimerImpl, TimerManager};
use crate::timer::manager::TimerImpl;
use crate::timer::TimerManager;

/**
 * Timer that represents the game-ticks. It will pause when the game is paused.
 *
 * @note Callbacks are executed in the game-thread.
 */
pub struct TimerGameTick {
    /// Monotonic counter, in ticks, since start of game.
    counter: TickCounter,
    timers: TimerManager<TimerGameTick>,
}

pub static mut GAME_CLOCK: TimerGameTick = TimerGameTick {
    counter: 0,
    timers: TimerManager::new(),
};

impl Clock for TimerGameTick {
    type Period = TickPeriod;
	type Instant = TickCounter;
	type Elapsed = u32;
	type Storage = TickStorage;

    fn manager(&mut self) -> &mut TimerManager<Self> {
        &mut self.timers
    }

    fn assert(&self, period: Self::Period) {
        if (period.priority == TickTimerPriority::None) {
            return;
        }
        /* Validate we didn't make a developer error and scheduled more than one
         * entry on the same priority. There can only be one timer on
         * a specific priority, to ensure we are deterministic, and to avoid
         * container sort order invariant issues with timer period saveload. */
        assert!(self.timers.timers()
                .iter()
                .all(|(k, _)| k.0.priority != period.priority)
        );
    }

    fn elapse(&mut self, delta: Self::Elapsed) -> bool {
        self.counter += 1;
        for (key, timer) in Self::manager().iter_mut() {
            match timer {
                TimerImpl::Interval { storage } => {
                    if timer.period() == 0 {
                        continue;
                    }

                    storage.elapsed += delta;

                    let mut count = 0;
                    while (storage.elapsed >= key.0.value) {
                        storage.elapsed -= key.0.value;
                        count += 1;
                    }

                    if (count > 0) {
                        key.1(count);
                    }
                }
                TimerImpl::Timeout { storage, has_fired } => {
                    if (has_fired) {
                        continue;
                    }
                    if (key.0.value == 0) {
                        continue;
                    }

                    storage.elapsed += delta;

                    if (storage.elapsed >= key.0.value) {
                        key.1();
                        *has_fired = true;
                    }
                }
            }
        }
        true
    }
}

/// The type to store ticks in
pub type Ticks = i32;
pub type TickCounter = u64;

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum TickTimerPriority {
    /// These timers can be executed in any order; the order is not relevant.
    None,
    /* For all other priorities, the order is important.
     * For safety, you can only set up a single timer on a single priority. */
    /// Considering starting a new competitor/AI.
    CompetitorTimeout,
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct TickPeriod {
    pub priority: TickTimerPriority,
    pub value: u32,
}

#[derive(Default)]
pub struct TickStorage {
    pub elapsed: u32,
}

impl TimerGameTick {
    /// Representation of an invalid number of ticks.
    pub const INVALID_TICKS: Ticks = -1;

    /**
     * 1 day is 74 ticks; TimerGameCalendar::date_fract used to be uint16_t and incremented by 885. On an overflow the new day begun and 65535 / 885 = 74.
     * 1 tick is approximately 27 ms.
     * 1 day is thus about 2 seconds (74 * 27 = 1998) on a machine that can run OpenTTD normally
     */
    /// ticks per day
    pub const DAY_TICKS: Ticks = 74;
    /// Estimation of how many ticks fit in a single second.
    pub const TICKS_PER_SECOND: Ticks = 1000 / MILLISECONDS_PER_TICK as Ticks;

    /// Cycle duration for updating station rating.
    pub const STATION_RATING_TICKS: Ticks = 185;
    /// Cycle duration for updating station acceptance.
    pub const STATION_ACCEPTANCE_TICKS: Ticks = 250;
    /// Cycle duration for cleaning dead links.
    pub const STATION_LINKGRAPH_TICKS: Ticks = 504;
    /// Cycle duration for aging cargo.
    pub const CARGO_AGING_TICKS: Ticks = 185;
    /// Cycle duration for industry production.
    pub const INDUSTRY_PRODUCE_TICKS: Ticks = 256;
    /// Cycle duration for towns trying to grow (this originates from the size of the town array in TTD).
    pub const TOWN_GROWTH_TICKS: Ticks = 70;
    /// Cycle duration for lumber mill's extra action.
    pub const INDUSTRY_CUT_TREE_TICKS: Ticks = Self::INDUSTRY_PRODUCE_TICKS * 2;
}
