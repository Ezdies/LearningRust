use crate::date::Date;
use crate::event::Event;
use crate::todo::ToDo;

use std::fs::File;
use std::io::{Error, Read, Result, Write};

pub struct Reader {
    event: Event,
    file: Option<File>,
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            event: Event::default(),
            file: None,
        }
    }

    pub fn set_file(&mut self, filename: &str) {
        let file = match File::open(filename) {
            Ok(file) => file,
            Err(err) => panic!("Failed to open file: {}", err),
        };
        self.file = Some(file);
    }

    pub fn read_from_file(&mut self) -> Result<(), String> {
        let file = match &self.file {
            Some(file) => file,
            None => return Err("No file set".to_string()),
        };

        let mut buffer = String::new();

        match file.read_to_string(&mut buffer) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to read from file: {}", err)),
        }
    }
}
