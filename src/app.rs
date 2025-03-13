use crate::gui;

use gtk4::{glib, gio,
	   prelude::*,
	   subclass::prelude::*};	 

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
    
    #[derive(Default)]
    pub struct App {
	pub window: RefCell<Option<gui::Window>>,
	
//	pub course_list: RefCell<Option<gio::>>;
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

	    // create and assign window field with a new window
	    *self.window.borrow_mut() = Some(gui::Window::new(&*self.obj()));

	    let window = self.window.borrow();

	    window.as_ref().unwrap().present();	    	    
	}
    }

    impl GtkApplicationImpl for App {}
}
