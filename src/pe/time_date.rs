use crate::*;

use bytemuck::*;

use std::fmt::{self, Debug, Formatter};
use std::time::{Duration, SystemTime};



/// Seconds elapsed since midnight, January 1, 1970, Universal Coordinated Time
///
/// "Y2K" issues to occur early Feb 7, 2106
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[derive(Pod, Zeroable)]
pub struct TimeDate(u32le);

impl TimeDate {
    pub const UNIX_EPOCH : TimeDate = TimeDate(u32le::new(0));
}

impl From<TimeDate> for SystemTime {
    fn from(value: TimeDate) -> Self {
        SystemTime::UNIX_EPOCH + Duration::from_secs(value.0.to_le().into())
    }
}

impl FromMemory for TimeDate {
    type Raw    = Self;
    type Error  = std::io::Error;
    fn from_raw(raw: Self::Raw) -> Result<Self, Self::Error> { Ok(raw) }
}

impl Debug for TimeDate {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if *self == Self::UNIX_EPOCH {
            return write!(fmt, "TimeDate::UNIX_EPOCH");
        }

        // This is not fully general, and only attempts to handle the full 32-bit range:
        //      0x00000000:  1970-01-01  0:00:00 UTC
        //      0xFFFFFFFF:  2106-02-07  6:28:15 UTC
        //
        // Leap years are:  1972 + 4n
        // With the exception of 2100, which is not a leap year.
        //
        // https://www.wolframalpha.com/input/?i=unix+epoch+%2B+%282**32-1%29+seconds

        let seconds_since_epoch = self.0.to_le();
        let days_since_epoch    = seconds_since_epoch / SECONDS_PER_DAY;

        let mut year = 1970 + 4 * (days_since_epoch / DAYS_PER_4ISH_YEARS);
        let mut day_idx = days_since_epoch % DAYS_PER_4ISH_YEARS + u32::from(year > 2100);
        let mut month = 1;

        while day_idx >= days_in_year(year) { // 0 .. 4 loops
            day_idx -= days_in_year(year);
            year += 1;
        }

        for month_len in [31, 28 + leap_days(year), 31, 30, 31, 30, 31, 31, 30, 31, 30, 31].iter().copied() {
            if day_idx < month_len { break }
            day_idx -= month_len;
            month += 1;
        }

        let seconds_since_midnight = seconds_since_epoch % SECONDS_PER_DAY;
        let second  = seconds_since_midnight % 60;
        let minute  = seconds_since_midnight / 60 % 60;
        let hour    = seconds_since_midnight / 60 / 60;

        write!(fmt, "TimeDate({: >4}-{:02}-{:02} {: >2}:{:02}:{:02} UTC)", year, month, day_idx+1, hour, minute, second)
    }
}

const SECONDS_PER_DAY : u32
    = 60    // seconds per minute
    * 60    // minutes per hour
    * 24;   // hours per day

/// Caveat: Every 100 years, we skip a leap year, except every 400 years, when we don't.
const DAYS_PER_4ISH_YEARS : u32 = 365 * 4 + 1;

const fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}

const fn leap_days(year: u32) -> u32 {
    is_leap_year(year) as u32
}

const fn days_in_year(year: u32) -> u32 {
    365 + leap_days(year)
}

#[test] fn leap_years() {
    assert_eq!(is_leap_year(1970), false);
    assert_eq!(is_leap_year(1971), false);
    assert_eq!(is_leap_year(1972), true);
    assert_eq!(is_leap_year(1973), false);
    assert_eq!(is_leap_year(1974), false);
    assert_eq!(is_leap_year(1975), false);
    assert_eq!(is_leap_year(1976), true);
    assert_eq!(is_leap_year(1977), false);
    assert_eq!(is_leap_year(2000), true);
    assert_eq!(is_leap_year(2100), false);
    assert_eq!(is_leap_year(2200), false);
    assert_eq!(is_leap_year(2300), false);
    assert_eq!(is_leap_year(2400), true);
}

#[test] fn times() {
    assert_eq!("TimeDate::UNIX_EPOCH",              format!("{:?}", TimeDate(u32le::new(0))));
    assert_eq!("TimeDate(1970-01-01  0:00:01 UTC)", format!("{:?}", TimeDate(u32le::new(1))));
    assert_eq!("TimeDate(1970-01-01 23:59:59 UTC)", format!("{:?}", TimeDate(u32le::new(SECONDS_PER_DAY-1))));
    assert_eq!("TimeDate(1970-01-02  0:00:00 UTC)", format!("{:?}", TimeDate(u32le::new(SECONDS_PER_DAY-0))));
    assert_eq!("TimeDate(2106-02-07  6:28:15 UTC)", format!("{:?}", TimeDate(u32le::new(!0))));
}

#[test] fn dates() {
    for (day, expected_debug_date_fmt) in [
        //(    0, "1970-01-01"), // TimeDate::UNIX_EPOCH
        (    1, "1970-01-02"),

        // end of month
        (   30, "1970-01-31"),
        (   31, "1970-02-01"),

        // end of year
        (  364, "1970-12-31"),
        (  365, "1971-01-01"),

        // 1970 is not a leap year
        (   58, "1970-02-28"),
        (   59, "1970-03-01"),

        // 1971 is not a leap year
        (  423, "1971-02-28"),
        (  424, "1971-03-01"),

        // 1972 *is* a leap year
        (  788, "1972-02-28"),
        (  789, "1972-02-29"),
        (  790, "1972-03-01"),

        // 1973 is not a leap year
        ( 1154, "1973-02-28"),
        ( 1155, "1973-03-01"),

        (10000, "1997-05-19"), // https://www.wolframalpha.com/input/?i=unix+epoch+%2B+10000+days

        // 2000 *is* a leap year
        (10956, "1999-12-31"),
        (10957, "2000-01-01"),
        (10957+58, "2000-02-28"),
        (10957+59, "2000-02-29"),
        (10957+60, "2000-03-01"),

        // https://www.wolframalpha.com/input/?i=%28jan+1st+2100+-+unix+epoch%29+in+days
        // 2100 is *not* a leap year
        (47481, "2099-12-31"),
        (47482, "2100-01-01"),
        (47540, "2100-02-28"),
        (47541, "2100-03-01"),

        // 2104 *is* a leap year
        (48941, "2103-12-31"),
        (48942, "2104-01-01"),
        (49000, "2104-02-28"),
        (49001, "2104-02-29"),
        (49002, "2104-03-01"),
    ].iter().copied() {
        let fmt = format!("{:?}", TimeDate(u32le::new(day * 24 * 60 * 60)));
        let fmt = fmt.strip_prefix("TimeDate(").unwrap();
        let fmt = fmt.strip_suffix("  0:00:00 UTC)").unwrap();
        assert!(
            expected_debug_date_fmt == fmt,
            concat!(
                "date format mismatch\n",
                "day:      {day}\n",
                "expected: {expected:?}\n",
                "actual:   {actual:?}\n",
            ),
            day     =day,
            expected=expected_debug_date_fmt,
            actual  =fmt,
        );
    }
}
