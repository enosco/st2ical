mod imp;
use crate::data;

use gtk4 as gtk;
use gtk::glib::{self, clone, Object};
use gtk::gio;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::Application;

use std::io::BufReader;
use std::fs::File;

// Establish custom class, UI, based on application window
glib::wrapper! {
    pub struct UI(ObjectSubclass<imp::UI>)
	@extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
	@implements gtk::Accessible, gtk::Buildable, gtk::Native;
}

impl UI {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setup_callbacks(&self) {	
	let ui = self.imp();

	ui.file_button.connect_clicked(clone!(
	    #[weak] ui,
	    move |_| {
		//Self::open_dialog(ui.obj().as_ref());
		ui.obj().open_dialog();
	    }));

	ui.save_button.connect_clicked(move |_| {
	    data::register_click("test")
	});

	ui.crn_entry.connect_activate(clone!(
	    #[weak] ui,
	    move |entry| {
		
		// make parse return Result<hashmap, err>
		let crn = entry.buffer()
		    .text()
		    .as_str()
		    .parse();

		if let Ok(crn) = crn {
		    ui.obj().load_course(crn);
		}
	}));
    }


    // some of this file manip might be redundant, check that

    fn open_dialog(&self) {
	let dialog = gtk::FileDialog::builder()
	    .accept_label("Open")
	    .modal(true)
	    .build();

	dialog.open(Some(self), gio::Cancellable::NONE, clone!(
	    #[weak(rename_to = ui)] self,
	    move |file| {
	    if let Ok(file) = file {
		
		ui.imp().file_label.set_label(
		    file.basename()
			.expect("ERR: invalid pathbuf")
			.to_str()
			.expect("ERR: cannot be converted to str"));
		
		Self::load_file(&ui, &file);
	    }
	}));
    }

    fn load_file(&self, file: &gio::File) {
	let mut db = self.imp().database.borrow_mut();
	
	if let Some(path) = file.path() {
	    //	    let reader = std::io::BufReader::new(path);
	    let file = File::open(path)
		.expect("ERR: could not open file");
	    
	    db.load_from_xml(BufReader::new(file));
	} else {
	    println!("invalid file path");
	}
	
	/*
	self.imp().database
	    .borrow_mut()
	.load_from_xml(&file.path().expect("ERR: invalid file path"));
	*/
    }

    fn load_course(&self, crn: i32) {
	let ui = self.imp();
	let db = ui.database.borrow();
		
	if let Some(event) = db.retrieve_course(crn) {	 
	    ui.days_field.set_text(&event.days);
	    ui.location_field.set_text(&event.location);
	} else {
	    println!("could not retrieve course");
	}
	
    }
}
