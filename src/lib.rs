pub type Year = u16;
pub type Month = u8;
pub type DayOfMonth = u8;
pub type DayOfYear = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    year: Year,
    month: Month,
    day: DayOfMonth,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidDate,
}

impl Date {
    const FIRST_DATE: Date = Date {
        year: 1582,
        month: 10,
        day: 15,
    };

    pub fn from_year_month_day(year: Year, month: Month, day: DayOfMonth) -> Result<Self, Error> {
        let date = Self { year, month, day };
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

    pub fn is_leap_year(&self) -> bool {
        (0 == self.year % 4) && (0 != (self.year % 100) || 0 == (self.year % 400))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_valid_date_from_year_month_day() {
        let date = Date::from_year_month_day(1582, 10, 15);
        assert!(date.is_ok());
    }

    #[test]
    fn create_invalid_invalid_from_year_month_day() {
        let date = Date::from_year_month_day(1582, 10, 14);
        assert_eq!(date.err(), Some(Error::InvalidDate));
    }

    #[test]
    fn is_leap_year() {
        assert!(Date::from_year_month_day(2000, 1, 1)
            .unwrap()
            .is_leap_year());
        assert!(!Date::from_year_month_day(2001, 1, 1)
            .unwrap()
            .is_leap_year());
        assert!(!Date::from_year_month_day(2002, 1, 1)
            .unwrap()
            .is_leap_year());
        assert!(!Date::from_year_month_day(2003, 1, 1)
            .unwrap()
            .is_leap_year());
        assert!(Date::from_year_month_day(2004, 1, 1)
            .unwrap()
            .is_leap_year());
        assert!(!Date::from_year_month_day(2100, 1, 1)
            .unwrap()
            .is_leap_year());
    }
}
