use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::io::Read;
use quick_xml::reader::Reader;


pub struct Event {
    pub days: String,
    pub location: String,
    pub time_start: String,
    pub time_end: String,
    pub date_start: String,
    pub date_end: String,
}

#[derive(Default)]
pub struct Database {
    course_data: HashMap<i32, Event>,  
}

impl Database {
    
    pub fn load_from_xml(&mut self, reader: impl Read) -> Result<(), Error> {
	return Ok(())
    }

    pub fn retrieve_course(&self, crn: i32) -> Option<&Event> {
	return self.course_data.get(&crn);
    }
}

pub fn register_click(name: &str) {
    println!("{} just clicked", name);
}



