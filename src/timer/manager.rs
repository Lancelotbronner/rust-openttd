use crate::clock::Clock;
use std::cmp::Ordering;
use std::collections::{btree_map, BTreeMap};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/**
 * The TimerManager manages a single Timer-type.
 *
 * It allows for automatic registration and unregistration of timers like Interval and OneShot.
 *
 * Each Timer-type needs to implement the Elapsed() method, and distribute that to the timers if needed.
 */
pub struct TimerManager<C: Clock> {
    storage: BTreeMap<TimerKey<C>, TimerImpl<C>>,
}

impl<C: Clock> TimerManager<C> {
    pub const fn new() -> Self {
        Self {
            storage: BTreeMap::new(),
        }
    }

    /**
     * Register a timer.
     *
     * @param timer The timer to register.
     */
    pub fn register(&mut self, key: TimerKey<C>, value: TimerImpl<C>) {
        C::assert(key.period.clone());
        self.storage.insert(key, value);
    }

    /**
     * Unregister a timer.
     *
     * @param timer The timer to unregister.
     */
    pub fn unregister(&mut self, key: &TimerKey<C>) -> Option<TimerImpl<C>> {
        self.storage.remove(key)
    }

    /**
    	* Change the period of a registered timer.
    	*
    	* @param timer The timer to change the period of.
    	* @param new_period The new period value.
    	*/
    pub fn change_period(&mut self, key: &mut TimerKey<C>, period: C::Period, reset: bool) {
        let timer = self
            .unregister(key)
            .and_then(|t| if reset { Some(t) } else { None })
            .unwrap_or(TimerImpl::Interval {
                storage: C::Storage::default(),
            });
        key.period = period;
        self.register((*key).clone(), timer);
    }

    pub(super) fn timers(&self) -> &BTreeMap<TimerKey<C>, TimerImpl<C>> {
        &self.storage
    }

    pub(super) fn iter_mut(&mut self) -> btree_map::IterMut<TimerKey<C>, TimerImpl<C>> {
        self.storage.iter_mut()
    }
}

pub struct TimerKey<C: Clock> {
    period: C::Period,
    callback: Rc<dyn Fn(u32) + 'static>,
}

impl<C: Clock> TimerKey<C> {
    fn ptr(&self) -> *const dyn Fn(u32) {
        self.callback.as_ref() as *const dyn Fn(u32)
    }
}

impl<C: Clock> Clone for TimerKey<C> {
    fn clone(&self) -> Self {
        TimerKey {
            period: self.period.clone(),
            callback: self.callback.clone(),
        }
    }
}

impl<C: Clock> Hash for TimerKey<C> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.period.hash(state);
        self.ptr().hash(state);
    }
}

impl<C: Clock> PartialEq<Self> for TimerKey<C> {
    fn eq(&self, other: &Self) -> bool {
        self.period.eq(&other.period) && self.ptr().eq(&other.ptr())
    }
}

impl<C: Clock> Eq for TimerKey<C> {}

impl<C: Clock> PartialOrd<Self> for TimerKey<C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.period == other.period {
            return self.ptr().partial_cmp(&other.ptr());
        }
        self.period.partial_cmp(&other.period)
    }
}

impl<C: Clock> Ord for TimerKey<C> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.period == other.period {
            return self.ptr().cmp(&other.ptr());
        }
        self.period.cmp(&other.period)
    }
}

impl<C: Clock> TimerKey<C> {
    pub fn call(&self, arg: u32) {
        self.callback.call((arg,));
    }
}

#[derive(Debug)]
pub enum TimerImpl<C: Clock> {
    Interval {
        storage: C::Storage,
    },
    Timeout {
        storage: C::Storage,
        has_fired: bool,
    },
}
