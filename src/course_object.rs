use std::cell::RefCell;
use gtk4::{glib, prelude::*};
use glib::{Object,
	   Properties,
	   subclass::prelude::*};

glib::wrapper! {
    pub struct CourseObject(ObjectSubclass<imp::CourseObject>);
}

impl CourseObject {
    pub fn new(crn: String, name: String)
	/*
	days: String
	location: String,
	time_start: String,
	time_end: String,
	date_start: String,
	date_end: String) -> Self {
	*/
	       -> Self {
	Object::builder()
	    .property("crn", crn)
	    .property("name", name)
	    /*
	    .property("location", location)
	    .property("time_start", time_start)
	    .property("time_end", time_end)
	    .property("date_start", date_start)
	    .property("date_end", date_end)
	    */
	    .build()
    }
}

#[derive(Default, Clone)]
pub struct CourseData {
    pub crn: String,
    pub name: String,
    /*
    pub days: String,
    pub location: String,
    pub time_start: String,
    pub time_end: String,
    pub date_start: String,
    pub date_end: String,
    */
}

mod imp {
    use super::*;
    
    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::CourseObject)]
    pub struct CourseObject {
	#[property(name = "crn", get, set, type = String, member = crn)]
	#[property(name = "name", get, set, type = String, member = name)]
	/*
	#[property(name = "location", get, set, type = String, member = location)]
	#[property(name = "time_start", get, set, type = String, member = time_start)]
	#[property(name = "time_end", get, set, type = String, member = time_end)]
	#[property(name = "date_start", get, set, type = String, member = date_start)]
	#[property(name = "date_end", get, set, type = String, member = date_end)]
	*/
	pub data: RefCell<CourseData>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CourseObject {
	const NAME: &'static str = "CourseObject";
	type Type = super::CourseObject;
    }
    
    #[glib::derived_properties]
    impl ObjectImpl for CourseObject {}    
}
