#[derive(Debug, PartialEq, PartialOrd, Clone, Eq, Ord)]
pub struct Date {
    day: u32,
    month: u32,
    year: u32,
}

impl Date {
    pub fn new(day: u32, month: u32, year: u32) -> Date {
        Date { day, month, year }
    }
    pub fn get_day(&self) -> u32 {
        self.day
    }
    pub fn get_month(&self) -> u32 {
        self.month
    }
    pub fn get_year(&self) -> u32 {
        self.year
    }
    pub fn print(&self) -> String {
        format!("{:02}-{:02}-{}", self.day, self.month, self.year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_construction() {
        let date = Date::new(23, 3, 2008);
        assert_eq!(date.get_day(), 23);
        assert_eq!(date.get_month(), 3);
        assert_eq!(date.get_year(), 2008);
    }

    #[test]
    fn printing() {
        let date1 = Date::new(11, 2, 2003);
        let date2 = Date::new(1, 12, 1996);
        assert_eq!(date1.print(), "11-02-2003");
        assert_eq!(date2.print(), "01-12-1996");
    }
}
