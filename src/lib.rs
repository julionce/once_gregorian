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

    const LEAP_MONTH_DAYS: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const NON_LEAP_MONTH_DAYS: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

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
        (0 == self.year % 4) && (0 != (self.year % 100) || 0 == (self.year % 400))
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
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => match self.is_leap_year() {
                true => 29,
                false => 28,
            },
            _ => 0,
        }
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

    #[test]
    fn year_days() {
        assert_eq!(
            Date::from_year_month_day(2000, 1, 1).unwrap().year_days(),
            366
        );
        assert_eq!(
            Date::from_year_month_day(2001, 1, 1).unwrap().year_days(),
            365
        );
    }

    #[test]
    fn month_days() {
        let leap_year_month_days: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        for (index, value) in leap_year_month_days.iter().enumerate() {
            let date = Date::from_year_month_day(2000, (index + 1) as u8, 1).unwrap();
            assert_eq!(date.month_days(), *value);
        }

        let non_leap_year_month_days: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        for (index, value) in non_leap_year_month_days.iter().enumerate() {
            let date = Date::from_year_month_day(2001, (index + 1) as u8, 1).unwrap();
            assert_eq!(date.month_days(), *value);
        }
    }
}
