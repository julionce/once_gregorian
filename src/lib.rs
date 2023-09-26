#![feature(const_trait_impl)]

pub type DayOfMonth = u8;
pub type DayOfYear = u16;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidDate,
    InvalidMonthNumber,
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

#[derive(Debug, Clone, Copy)]
struct GenericYear<const F: DayOfMonth> {
    inner: u16,
}

impl<const F: DayOfMonth> GenericYear<F> {
    const MONTH_DAYS: [DayOfMonth; 12] = [31, F, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

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
}

impl<const F: DayOfMonth> const Into<u16> for GenericYear<F> {
    fn into(self) -> u16 {
        self.inner
    }
}

impl<const F: DayOfMonth> const Into<GenericYear<F>> for u16 {
    fn into(self) -> GenericYear<F> {
        GenericYear::<F> { inner: self }
    }
}

type LeapYear = GenericYear<29u8>;
type NonLeapYear = GenericYear<28u8>;

#[derive(Debug, Clone, Copy)]
enum InternalYear {
    LeapYear(GenericYear<29u8>),
    NonLeapYear(GenericYear<28u8>),
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

    const LEAP_MONTH_DAYS: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const NON_LEAP_MONTH_DAYS: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

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

    pub fn year(&self) -> Year {
        self.year
    }

    pub fn month(&self) -> Month {
        self.month
    }

    pub fn day_of_month(&self) -> DayOfMonth {
        self.day
    }

    const fn accumulate_month_days(is_leap_year: bool, month: u8) -> DayOfYear {
        match month {
            1 => 31,
            _ => match is_leap_year {
                true => {
                    Self::accumulate_month_days(is_leap_year, month - 1)
                        + Self::LEAP_MONTH_DAYS[(month - 1) as usize] as u16
                }
                false => {
                    Self::accumulate_month_days(is_leap_year, month - 1)
                        + Self::NON_LEAP_MONTH_DAYS[(month - 1) as usize] as u16
                }
            },
        }
    }

    pub fn is_leap_year(&self) -> bool {
        self.year.is_leap()
    }

    pub fn year_days(&self) -> DayOfYear {
        const LEAD_YEAR_DAYS: DayOfYear = Date::accumulate_month_days(true, 12);
        const NON_LEAD_YEAR_DAYS: DayOfYear = Date::accumulate_month_days(false, 12);
        match self.is_leap_year() {
            true => LEAD_YEAR_DAYS,
            false => NON_LEAD_YEAR_DAYS,
        }
    }

    pub fn month_days(&self) -> DayOfMonth {
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
