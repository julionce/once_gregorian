pub type Day = u8;
pub type DayOfYear = u16;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidDate,
    InvalidMonthNumber,
    InvalidDay,
    InvalidDayOfYear,
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
    pub const VALUES: [Self; 12] = [
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

impl Into<u8> for Month {
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

impl TryInto<Month> for u8 {
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

mod generic {

    use crate::{Day, Month};
    use std::ops::RangeInclusive;

    #[derive(Debug, Clone, Copy)]
    pub struct Year<const LEAP: bool> {
        pub inner: u16,
    }

    impl<const LEAP: bool> Year<LEAP> {
        const MONTH_DAYS: [Day; 12] = [
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

        pub const fn month_days(month: Month) -> Day {
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

        const fn first_of_month(month: Month) -> crate::DayOfYear {
            match month {
                Month::January => 1,
                m => {
                    Self::first_of_month(m.prev()) + Self::month_days(m.prev()) as crate::DayOfYear
                }
            }
        }

        const fn last_of_month(month: Month) -> crate::DayOfYear {
            match month {
                Month::January => Self::month_days(Month::January) as crate::DayOfYear,
                m => Self::last_of_month(m.prev()) + Self::month_days(m) as crate::DayOfYear,
            }
        }

        pub const TOTAL_DAYS: crate::DayOfYear = Self::last_of_month(Month::December);

        const fn range_of_month(month: Month) -> RangeInclusive<crate::DayOfYear> {
            Self::first_of_month(month)..=Self::last_of_month(month)
        }

        const fn find_month_helper(day_of_year: crate::DayOfYear, month: Month) -> Option<Month> {
            let range = Self::range_of_month(month);
            match *range.start() <= day_of_year && *range.end() >= day_of_year {
                true => Some(month),
                false => match month {
                    Month::December => None,
                    _ => Self::find_month_helper(day_of_year, month.next()),
                },
            }
        }

        const fn find_month(day_of_year: crate::DayOfYear) -> Option<Month> {
            Self::find_month_helper(day_of_year, Month::January)
        }

        pub const fn day_of_year_to_month_and_day(
            day_of_year: crate::DayOfYear,
        ) -> Option<(Month, Day)> {
            match Self::find_month(day_of_year) {
                Some(month) => Some((
                    month,
                    (day_of_year - Self::month_days(month) as crate::DayOfYear) as Day,
                )),
                None => None,
            }
        }
    }

    impl<const LEAP: bool> Into<u16> for Year<LEAP> {
        fn into(self) -> u16 {
            self.inner
        }
    }

    impl<const LEAP: bool> Into<Year<LEAP>> for u16 {
        fn into(self) -> Year<LEAP> {
            Year::<LEAP> { inner: self }
        }
    }
}

type LeapYear = generic::Year<true>;
type NonLeapYear = generic::Year<false>;

#[derive(Debug, Clone, Copy)]
enum InternalYear {
    LeapYear(LeapYear),
    NonLeapYear(NonLeapYear),
}

impl InternalYear {
    const fn month_days(&self, month: Month) -> Day {
        match self {
            InternalYear::LeapYear(_) => LeapYear::month_days(month),
            InternalYear::NonLeapYear(_) => NonLeapYear::month_days(month),
        }
    }

    const fn day_of_year_to_month_and_day(&self, day_of_year: DayOfYear) -> Option<(Month, Day)> {
        match self {
            InternalYear::LeapYear(_) => LeapYear::day_of_year_to_month_and_day(day_of_year),
            InternalYear::NonLeapYear(_) => NonLeapYear::day_of_year_to_month_and_day(day_of_year),
        }
    }
}

impl Into<u16> for InternalYear {
    fn into(self) -> u16 {
        match self {
            Self::LeapYear(y) => y.into(),
            Self::NonLeapYear(y) => y.into(),
        }
    }
}

impl PartialEq for InternalYear {
    fn eq(&self, other: &Self) -> bool {
        <InternalYear as Into<u16>>::into(*self) == <InternalYear as Into<u16>>::into(*other)
    }
}

impl PartialOrd for InternalYear {
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Year {
    inner: InternalYear,
}

impl Year {
    pub const fn new(year: u16) -> Self {
        if (0 == year % 4) && (0 != (year % 100) || 0 == (year % 400)) {
            Year {
                inner: InternalYear::LeapYear(LeapYear { inner: year }),
            }
        } else {
            Year {
                inner: InternalYear::NonLeapYear(NonLeapYear { inner: year }),
            }
        }
    }

    pub const fn is_leap(&self) -> bool {
        match self.inner {
            InternalYear::LeapYear(_) => true,
            InternalYear::NonLeapYear(_) => false,
        }
    }

    pub const fn month_days(&self, month: Month) -> Day {
        self.inner.month_days(month)
    }

    pub const fn total_days(&self) -> DayOfYear {
        match self.inner {
            InternalYear::LeapYear(_) => LeapYear::TOTAL_DAYS,
            InternalYear::NonLeapYear(_) => NonLeapYear::TOTAL_DAYS,
        }
    }

    const fn day_of_year_to_month_and_day(&self, day_of_year: DayOfYear) -> Option<(Month, Day)> {
        self.inner.day_of_year_to_month_and_day(day_of_year)
    }
}

impl Into<u16> for Year {
    fn into(self) -> u16 {
        self.inner.into()
    }
}

impl Into<Year> for u16 {
    fn into(self) -> Year {
        Year::new(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Date {
    year: Year,
    month: Month,
    day: Day,
}

impl Date {
    pub const fn year(&self) -> Year {
        self.year
    }

    pub const fn month(&self) -> Month {
        self.month
    }

    pub const fn day_of_month(&self) -> Day {
        self.day
    }

    pub const fn is_leap_year(&self) -> bool {
        self.year.is_leap()
    }

    pub const fn year_days(&self) -> DayOfYear {
        self.year.total_days()
    }

    pub const fn month_days(&self) -> Day {
        self.year.month_days(self.month)
    }
}

pub const FIRST_DATE: Date = Date {
    year: Year {
        inner: InternalYear::NonLeapYear(generic::Year::<false> { inner: 1582 }),
    },
    month: Month::October,
    day: 15,
};

enum InternalDateBuilder {
    MonthAndDay(Month, Day),
    DayOfYear(DayOfYear),
}

pub struct DateBuilder {
    year: u16,
    date: InternalDateBuilder,
}

impl DateBuilder {
    pub const fn new() -> Self {
        DateBuilder {
            year: 2000,
            date: InternalDateBuilder::MonthAndDay(Month::January, 1),
        }
    }

    pub const fn year(mut self, year: u16) -> Self {
        self.year = year;
        self
    }

    pub const fn month(mut self, month: Month) -> Self {
        match self.date {
            InternalDateBuilder::MonthAndDay(_, day) => {
                self.date = InternalDateBuilder::MonthAndDay(month, day)
            }
            InternalDateBuilder::DayOfYear(_) => {
                self.date = InternalDateBuilder::MonthAndDay(month, 1)
            }
        }
        self
    }

    pub const fn day(mut self, day: Day) -> Self {
        match self.date {
            InternalDateBuilder::MonthAndDay(month, _) => {
                self.date = InternalDateBuilder::MonthAndDay(month, day)
            }
            InternalDateBuilder::DayOfYear(_) => {
                self.date = InternalDateBuilder::MonthAndDay(Month::January, day)
            }
        }
        self
    }

    pub const fn day_of_year(mut self, day_of_year: DayOfYear) -> Self {
        self.date = InternalDateBuilder::DayOfYear(day_of_year);
        self
    }

    pub fn build(&self) -> Result<Date, Error> {
        let year = Year::new(self.year);
        let (month, day) = match self.date {
            InternalDateBuilder::MonthAndDay(month, day) => {
                let total_month_days = year.month_days(month);
                match 1 <= day && total_month_days >= day {
                    true => (month, day),
                    false => return Err(Error::InvalidDay),
                }
            }
            InternalDateBuilder::DayOfYear(day_of_year) => {
                match year.day_of_year_to_month_and_day(day_of_year) {
                    Some((month, day)) => (month, day),
                    None => return Err(Error::InvalidDayOfYear),
                }
            }
        };
        let date = Date { year, month, day };
        if date >= FIRST_DATE {
            Ok(date)
        } else {
            Err(Error::InvalidDate)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_first_date() {
        let date = DateBuilder::new()
            .year(1582)
            .month(Month::October)
            .day(15)
            .build();
        assert!(date.is_ok());
    }

    #[test]
    fn build_date_previous_to_first_date() {
        let date = DateBuilder::new()
            .year(1582)
            .month(Month::October)
            .day(14)
            .build();
        assert_eq!(date.err(), Some(Error::InvalidDate));
    }

    #[test]
    fn is_leap_year() {
        assert!(DateBuilder::new()
            .year(2000)
            .month(Month::January)
            .day(1)
            .build()
            .unwrap()
            .is_leap_year());
        assert!(!DateBuilder::new()
            .year(2001)
            .month(Month::January)
            .day(1)
            .build()
            .unwrap()
            .is_leap_year());
        assert!(!DateBuilder::new()
            .year(2002)
            .month(Month::January)
            .day(1)
            .build()
            .unwrap()
            .is_leap_year());
        assert!(!DateBuilder::new()
            .year(2003)
            .month(Month::January)
            .day(1)
            .build()
            .unwrap()
            .is_leap_year());
        assert!(DateBuilder::new()
            .year(2004)
            .month(Month::January)
            .day(1)
            .build()
            .unwrap()
            .is_leap_year());
        assert!(!DateBuilder::new()
            .year(2100)
            .month(Month::January)
            .day(1)
            .build()
            .unwrap()
            .is_leap_year());
    }

    #[test]
    fn year_days() {
        assert_eq!(
            DateBuilder::new()
                .year(2000)
                .month(Month::January)
                .day(1)
                .build()
                .unwrap()
                .year_days(),
            366
        );
        assert_eq!(
            DateBuilder::new()
                .year(2001)
                .month(Month::January)
                .day(1)
                .build()
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
