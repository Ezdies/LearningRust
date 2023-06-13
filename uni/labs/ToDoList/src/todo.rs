use crate::event::Event;

#[derive(Debug, PartialEq, PartialOrd, Clone, Eq, Ord)]
pub struct ToDo {
    events: Vec<Event>,
}

impl ToDo {
    pub fn new() -> ToDo {
        ToDo { events: Vec::new() }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn delete_event(&mut self, event: Event) {
        let index = self.events.iter().position(|e| *e == event);
        self.events.remove(index.unwrap_or_default());
    }

    pub fn print_list(&self) -> String {
        if !self.events.is_empty() {
            self.events.iter().map(|event| event.print_info()).collect()
        } else {
            "No events found".to_string()
        }
    }

    fn get_events(&self) -> Vec<Event> {
        self.events.clone()
    }
}

#[cfg(test)]

mod tests {
    use crate::date::Date;
    use crate::event::Event;
    use crate::todo::ToDo;

    #[test]
    fn null_construction() {
        let list = ToDo::new();
        assert_eq!(list.get_events().len(), 0);
    }
    #[test]
    fn adding_events() {
        let mut list = ToDo::new();
        list.add_event(Event::new(
            "Description".to_string(),
            Date::new(21, 3, 1996),
        ));
        assert_eq!(list.get_events()[0].get_date(), Date::new(21, 3, 1996));
        assert_eq!(list.get_events()[0].get_description(), "Description");
    }

    #[test]
    fn deleting_events() {
        let mut list = ToDo::new();
        let event1 = Event::new("Event description".to_string(), Date::new(21, 3, 1023));
        let event2 = Event::new("Event description".to_string(), Date::new(30, 4, 2012));

        list.add_event(event1.clone());
        list.add_event(event2.clone());

        list.delete_event(event1);

        assert_eq!(list.print_list(), list.get_events()[0].print_info());
    }

    #[test]
    fn print_event_list() {
        let mut list = ToDo::new();
        let event1 = Event::new("Event description".to_string(), Date::new(30, 1, 2008));
        let event2 = Event::new("Event description2".to_string(), Date::new(21, 2, 2011));

        list.add_event(event1.clone());
        list.add_event(event2.clone());

        assert_eq!(
            format!("{}{}", event1.print_info(), event2.print_info()),
            list.print_list()
        );
    }
}
