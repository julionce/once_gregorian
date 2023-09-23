pub type Year = u16;
pub type Month = u8;
pub type DayOfMonth = u8;
pub type DayOfYear = u16;

#[derive(Debug, Clone, Copy)]
pub struct Date {
    year: Year,
    month: Month,
    day: DayOfMonth,
}

impl Date {
    pub fn from_year_month_day(year: Year, month: Month, day: DayOfMonth) -> Self {
        Self { year, month, day }
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
}

#[cfg(test)]
mod tests {
    use super::*;
}
