/*
 * This file is part of OpenTTD.
 * OpenTTD is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, version 2.
 * OpenTTD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details. You should have received a copy of the GNU General Public License along with OpenTTD. If not, see <https://www.gnu.org/licenses/old-licenses/gpl-2.0>.
 */
use crate::timer::{Date, DateFrac, Day, Month, Year, YearMonthDay};

/** @file timer_game_calendar.h Definition of the game-calendar-timer. */

#[derive(Default)]
pub struct Calendar;

/**
 * Timer that is increased every 27ms, and counts towards ticks / days / months / years.
 *
 * The amount of days in a month depends on the month and year (leap-years).
 * There are always 74 ticks in a day (and with 27ms, this makes 1 day 1.998 seconds).
 *
 * Calendar time is used for technology and time-of-year changes, including:
 * - Vehicle, airport, station, object introduction and obsolescence
 * - NewGRF variables for visual styles or behavior based on year or time of year (e.g. variable snow line)
 * - Inflation, since it is tied to original game years. One interpretation of inflation is that it compensates for faster and higher capacity vehicles,
 *   another is that it compensates for more established companies. Each of these point to a different choice of calendar versus economy time, but we have to pick one
 *   so we follow a previous decision to tie inflation to original TTD game years.
 */
#[derive(Default)]
pub struct TimerGameCalendar {
    ///< Current year, starting at 0.
    pub year: Year<Calendar>,
    /// Current month (0..11).
    pub month: Month<Calendar>,
    /// Current date in days (day counter).
    pub date: Date<Calendar>,
    /// Fractional part of the day.
    pub date_fract: DateFrac<Calendar>,
    /// Subpart of date_fract that we use when calendar days are slower than economy days.
    pub sub_date_fract: u16,
}

pub static mut TIMER_GAME_CALENDAR: TimerGameCalendar = TimerGameCalendar::default();

impl TimerGameCalendar {
    pub fn ConvertDateToYMD(date: Date<Calendar>) -> YearMonthDay<Calendar> {}
    pub fn ConvertYMDToDate(
        year: Year<Calendar>,
        month: Month<Calendar>,
        day: Day<Calendar>,
    ) -> Date<Calendar> {
    }
    pub fn SetDate(date: Date<Calendar>, fract: DateFrac<Calendar>) {}
}

/**
 * Storage class for Calendar time constants.
 */
pub struct CalendarTime;

impl CalendarTime {
    pub const DEF_MINUTES_PER_YEAR: i32 = 12;
    pub const FROZEN_MINUTES_PER_YEAR: i32 = 0;
    /// One week of real time. The actual max that doesn't overflow TimerGameCalendar::sub_date_fract is 10627, but this is neater.
    pub const MAX_MINUTES_PER_YEAR: i32 = 10080;
}
