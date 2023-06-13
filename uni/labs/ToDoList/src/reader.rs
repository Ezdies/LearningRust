use crate::date::Date;
use crate::todo::ToDo;
use crate::event::Event;

pub struct Reader{
    event : Event,
    date: Date,
}

impl Reader {
    fn new()-> Reader{
        Reader {event: Event, date: Date}
    }
}
