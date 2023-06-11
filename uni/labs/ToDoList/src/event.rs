use crate::date::Date;

#[derive(Debug, PartialEq, PartialOrd, Clone, Eq, Ord)]
pub struct Event {
    description: String,
    date: Date,
}

impl Event {
    pub fn new(description: String, date: Date) -> Event {
        Event { description, date }
    }
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
    pub fn get_date(&self) -> Date {
        self.date.clone()
    }
    pub fn print_info(&self) -> String {
        format!("{}\n{}", self.get_date().print(), self.get_description())
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn correct_event_construction() {
        let date = Date::new(23, 3, 2008);
        let description = "Opis wydarzenia".to_string();
        let event = Event::new(description.clone(), date.clone());
        assert_eq!(event.get_date(), date);
        assert_eq!(event.get_description(), description);
    }

    #[test]
    fn printing_event() {
        let event = Event::new("Desc".to_string().clone(), Date::new(28, 4, 2012).clone());
        assert_eq!(event.print_info(), "28-04-2012\nDesc");
    }
}
