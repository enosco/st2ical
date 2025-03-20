use crate::data;
use crate::gui;
use crate::course_object;

use gui::{Window, FileChooserHub};
//use data::Database;
use course_object::CourseObject;

use gtk4::{glib, gio,
	   prelude::*,
	   subclass::prelude::*};	 

use glib::closure_local;

glib::wrapper! {
    pub struct App(ObjectSubclass<imp::App>)
	@extends gio::Application, gtk4::Application,
    @implements gio::ActionMap, gio::ActionGroup;
}

impl App {
    pub fn new(app_id: &str) -> Self {
	glib::Object::builder()
	    .property("application_id", app_id)
	    .build()
    }
}
mod imp {
    use super::*;
    use std::cell::RefCell;
    use std::cell::Cell;
    use std::rc::Rc;
    
    #[derive(Default)]
    pub struct App {	
	pub window: RefCell<Option<gui::Window>>,
	pub course_data: Rc<RefCell<Option<gio::ListStore>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for App {
	const NAME: &'static str = "App";
	type Type = super::App;
	type ParentType = gtk4::Application;
    }

    impl ObjectImpl for App {}

    impl ApplicationImpl for App {
	fn activate(&self) {
	    self.parent_activate();
	    
	    let window = self.window.borrow();
	    let db_clone = self.course_data.clone();
	    
	    let file_hub = window.as_ref()
		.unwrap()
		.file_hub();

	    let file_view = window.as_ref()
		.unwrap()
		.file_view();
	    
	    file_hub.connect_closure(
		"load-button-clicked",
		false,				
		closure_local!(
		    move |file_hub: FileChooserHub| {
			
			if let Some(file) = file_hub.file() {

			    let path = file.path()
				.expect("Err: cannot retrieve path");				
			    
			    if let Ok(store) = data::parse_xlsx(&path) {
				*db_clone.borrow_mut() = Some(store);
				
			    }
			}

			for obj in db_clone.borrow().as_ref().unwrap() {
			    println!("{:?}", obj);
			}
		    }		    
		)
	    );
	    
	    window.as_ref().unwrap().present();	    
	}

	fn startup(&self) {
	    self.parent_startup();
	    
	    let window = gui::Window::new(&*self.obj());
	    let course_data = gio::ListStore::new::<CourseObject>();

	    let sel = gtk4::SingleSelection::new(Some(course_data));
	    window.file_view().set_model(Some(&sel));
	   
	    *self.window.borrow_mut() = Some(gui::Window::new(&*self.obj()));
	    *self.course_data.borrow_mut() = Some(gio::ListStore::new::<CourseObject>());
        }	
    }

    impl GtkApplicationImpl for App {}

    impl App {}
}
