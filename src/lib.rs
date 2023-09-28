#![feature(const_trait_impl)]
#![feature(const_cmp)]

use std::ops::RangeInclusive;

pub type DayOfMonth = u8;
pub type DayOfYear = u16;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidDate,
    InvalidMonthNumber,
    InvalidDay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    const VALUES: [Self; 12] = [
        Self::January,
        Self::February,
        Self::March,
        Self::April,
        Self::May,
        Self::June,
        Self::July,
        Self::August,
        Self::September,
        Self::October,
        Self::November,
        Self::December,
    ];

    pub const fn next(&self) -> Self {
        match self {
            Self::January => Self::February,
            Self::February => Self::March,
            Self::March => Self::April,
            Self::April => Self::May,
            Self::May => Self::June,
            Self::June => Self::July,
            Self::July => Self::August,
            Self::August => Self::September,
            Self::September => Self::October,
            Self::October => Self::November,
            Self::November => Self::December,
            Self::December => Self::January,
        }
    }

    pub const fn prev(&self) -> Self {
        match self {
            Self::January => Self::December,
            Self::February => Self::January,
            Self::March => Self::February,
            Self::April => Self::March,
            Self::May => Self::April,
            Self::June => Self::May,
            Self::July => Self::June,
            Self::August => Self::July,
            Self::September => Self::August,
            Self::October => Self::September,
            Self::November => Self::October,
            Self::December => Self::November,
        }
    }
}

impl const Into<u8> for Month {
    fn into(self) -> u8 {
        match self {
            Month::January => 1,
            Month::February => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        }
    }
}

impl const TryInto<Month> for u8 {
    type Error = Error;

    fn try_into(self) -> Result<Month, Self::Error> {
        match self {
            1 => Ok(Month::January),
            2 => Ok(Month::February),
            3 => Ok(Month::March),
            4 => Ok(Month::April),
            5 => Ok(Month::May),
            6 => Ok(Month::June),
            7 => Ok(Month::July),
            8 => Ok(Month::August),
            9 => Ok(Month::September),
            10 => Ok(Month::October),
            11 => Ok(Month::November),
            12 => Ok(Month::December),
            _ => Err(Error::InvalidMonthNumber),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct GenericYear<const LEAP: bool> {
    inner: u16,
}

impl<const LEAP: bool> GenericYear<LEAP> {
    const MONTH_DAYS: [DayOfMonth; 12] = [
        31,
        if LEAP { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];

    const fn month_days(month: Month) -> DayOfMonth {
        match month {
            Month::January => Self::MONTH_DAYS[0],
            Month::February => Self::MONTH_DAYS[1],
            Month::March => Self::MONTH_DAYS[2],
            Month::April => Self::MONTH_DAYS[3],
            Month::May => Self::MONTH_DAYS[4],
            Month::June => Self::MONTH_DAYS[5],
            Month::July => Self::MONTH_DAYS[6],
            Month::August => Self::MONTH_DAYS[7],
            Month::September => Self::MONTH_DAYS[8],
            Month::October => Self::MONTH_DAYS[9],
            Month::November => Self::MONTH_DAYS[10],
            Month::December => Self::MONTH_DAYS[11],
        }
    }

    const fn first_of_month(month: Month) -> DayOfYear {
        match month {
            Month::January => 1,
            m => Self::first_of_month(m.prev()) + Self::month_days(m.prev()) as DayOfYear,
        }
    }

    const fn last_of_month(month: Month) -> DayOfYear {
        match month {
            Month::January => Self::month_days(Month::January) as DayOfYear,
            m => Self::last_of_month(m.prev()) + Self::month_days(m) as DayOfYear,
        }
    }

    const TOTAL_DAYS: DayOfYear = Self::last_of_month(Month::December);

    const fn range_of_month(month: Month) -> RangeInclusive<DayOfYear> {
        Self::first_of_month(month)..=Self::last_of_month(month)
    }

    const fn find_month_helper(day_of_year: DayOfYear, month: Month) -> Option<Month> {
        let range = Self::range_of_month(month);
        match day_of_year.ge(range.start()) && day_of_year.le(range.end()) {
            true => Some(month),
            false => match month {
                Month::December => None,
                _ => Self::find_month_helper(day_of_year, month.next()),
            },
        }
    }

    const fn find_month(day_of_year: DayOfYear) -> Option<Month> {
        Self::find_month_helper(day_of_year, Month::January)
    }

    const fn to_month_and_day(day_of_year: DayOfYear) -> Option<(Month, DayOfMonth)> {
        match Self::find_month(day_of_year) {
            Some(month) => Some((
                month,
                (day_of_year - Self::month_days(month) as DayOfYear) as DayOfMonth,
            )),
            None => None,
        }
    }
}

impl<const LEAP: bool> const Into<u16> for GenericYear<LEAP> {
    fn into(self) -> u16 {
        self.inner
    }
}

impl<const LEAP: bool> const Into<GenericYear<LEAP>> for u16 {
    fn into(self) -> GenericYear<LEAP> {
        GenericYear::<LEAP> { inner: self }
    }
}

type LeapYear = GenericYear<true>;
type NonLeapYear = GenericYear<false>;

#[derive(Debug, Clone, Copy)]
enum InternalYear {
    LeapYear(GenericYear<true>),
    NonLeapYear(GenericYear<false>),
}

impl InternalYear {
    const fn month_days(&self, month: Month) -> DayOfMonth {
        match self {
            InternalYear::LeapYear(_) => LeapYear::month_days(month),
            InternalYear::NonLeapYear(_) => NonLeapYear::month_days(month),
        }
    }
}

impl const Into<u16> for InternalYear {
    fn into(self) -> u16 {
        match self {
            Self::LeapYear(y) => y.into(),
            Self::NonLeapYear(y) => y.into(),
        }
    }
}

impl const PartialEq for InternalYear {
    fn eq(&self, other: &Self) -> bool {
        <InternalYear as Into<u16>>::into(*self) == <InternalYear as Into<u16>>::into(*other)
    }
}

impl const PartialOrd for InternalYear {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs: u16 = (*self).into();
        let rhs: u16 = (*other).into();
        if lhs == rhs {
            Some(std::cmp::Ordering::Equal)
        } else if lhs < rhs {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }
}

struct GenericMonthAndDay<const LEAP: bool> {
    month: Month,
    day: DayOfMonth,
}

impl<const LEAP: bool> GenericMonthAndDay<LEAP> {
    const fn new(month: Month, day: DayOfMonth) -> Result<GenericMonthAndDay<LEAP>, Error> {
        let month_days = GenericYear::<LEAP>::month_days(month);
        match day.ge(&1) && day.le(&month_days) {
            true => Ok(Self { month, day }),
            false => Err(Error::InvalidDay),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Year {
    inner: InternalYear,
}

impl Year {
    pub const fn new(inner: u16) -> Self {
        inner.into()
    }

    pub const fn is_leap(&self) -> bool {
        match self.inner {
            InternalYear::LeapYear(_) => true,
            InternalYear::NonLeapYear(_) => false,
        }
    }

    pub const fn month_days(&self, month: Month) -> DayOfMonth {
        self.inner.month_days(month)
    }

    pub const fn total_days(&self) -> DayOfYear {
        match self.inner {
            InternalYear::LeapYear(_) => LeapYear::TOTAL_DAYS,
            InternalYear::NonLeapYear(_) => NonLeapYear::TOTAL_DAYS,
        }
    }
}

impl const Into<u16> for Year {
    fn into(self) -> u16 {
        self.inner.into()
    }
}

impl const Into<Year> for u16 {
    fn into(self) -> Year {
        if (0 == self % 4) && (0 != (self % 100) || 0 == (self % 400)) {
            Year {
                inner: InternalYear::LeapYear(self.into()),
            }
        } else {
            Year {
                inner: InternalYear::NonLeapYear(self.into()),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Date {
    year: Year,
    month: Month,
    day: DayOfMonth,
}

impl Date {
    const FIRST_DATE: Date = Date {
        year: Year::new(1582),
        month: Month::October,
        day: 15,
    };

    pub fn from_year_month_day(year: u16, month: Month, day: DayOfMonth) -> Result<Self, Error> {
        let date = Self {
            year: year.into(),
            month,
            day,
        };
        if date >= Self::FIRST_DATE {
            Ok(date)
        } else {
            Err(Error::InvalidDate)
        }
    }

    pub const fn year(&self) -> Year {
        self.year
    }

    pub const fn month(&self) -> Month {
        self.month
    }

    pub const fn day_of_month(&self) -> DayOfMonth {
        self.day
    }

    pub const fn is_leap_year(&self) -> bool {
        self.year.is_leap()
    }

    pub const fn year_days(&self) -> DayOfYear {
        self.year.total_days()
    }

    pub const fn month_days(&self) -> DayOfMonth {
        self.year.month_days(self.month)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_valid_date_from_year_month_day() {
        let date = Date::from_year_month_day(1582, Month::October, 15);
        assert!(date.is_ok());
    }

    #[test]
    fn create_invalid_invalid_from_year_month_day() {
        let date = Date::from_year_month_day(1582, Month::October, 14);
        assert_eq!(date.err(), Some(Error::InvalidDate));
    }

    #[test]
    fn is_leap_year() {
        assert!(Date::from_year_month_day(2000, Month::January, 1)
            .unwrap()
            .is_leap_year());
        assert!(!Date::from_year_month_day(2001, Month::January, 1)
            .unwrap()
            .is_leap_year());
        assert!(!Date::from_year_month_day(2002, Month::January, 1)
            .unwrap()
            .is_leap_year());
        assert!(!Date::from_year_month_day(2003, Month::January, 1)
            .unwrap()
            .is_leap_year());
        assert!(Date::from_year_month_day(2004, Month::January, 1)
            .unwrap()
            .is_leap_year());
        assert!(!Date::from_year_month_day(2100, Month::January, 1)
            .unwrap()
            .is_leap_year());
    }

    #[test]
    fn year_days() {
        assert_eq!(
            Date::from_year_month_day(2000, Month::January, 1)
                .unwrap()
                .year_days(),
            366
        );
        assert_eq!(
            Date::from_year_month_day(2001, Month::January, 1)
                .unwrap()
                .year_days(),
            365
        );
    }

    #[test]
    fn month_days() {
        let leap_year_month_days: [(Month, u8); 12] = [
            (Month::January, 31),
            (Month::February, 29),
            (Month::March, 31),
            (Month::April, 30),
            (Month::May, 31),
            (Month::June, 30),
            (Month::July, 31),
            (Month::August, 31),
            (Month::September, 30),
            (Month::October, 31),
            (Month::November, 30),
            (Month::December, 31),
        ];
        for value in leap_year_month_days {
            assert_eq!(LeapYear::month_days(value.0), value.1);
        }

        let non_leap_year_month_days: [(Month, u8); 12] = [
            (Month::January, 31),
            (Month::February, 28),
            (Month::March, 31),
            (Month::April, 30),
            (Month::May, 31),
            (Month::June, 30),
            (Month::July, 31),
            (Month::August, 31),
            (Month::September, 30),
            (Month::October, 31),
            (Month::November, 30),
            (Month::December, 31),
        ];
        for value in non_leap_year_month_days {
            assert_eq!(NonLeapYear::month_days(value.0), value.1);
        }
    }
}
