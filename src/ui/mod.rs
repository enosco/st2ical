mod imp;
use crate::data::*;

use gtk4 as gtk;
use gtk::glib::{self, clone, Object};
use gtk::gio;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::Application;//, FileChooserNative, FileChooserAction};
//use std::borrow:

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
		Self::open_dialog(ui.obj().as_ref());
	    }));
	 

	ui.crn_entry.connect_activate(move |entry| {

	    // make parse return Result<hashmap, err>
	    let crn = entry.buffer().text();


	});
    }

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
		
		Self::load_file(&file);
	    }
	}));
		
    }
  
    fn load_file(file: &gio::File) {
		
    }




	/*let file = dialog.file()
		    .expect("ERR: failed to open file");

		window.imp().file_label.set_label(
		    file.basename()
			.expect("ERR: invalid pathbuf")
			.to_str()
			.expect("ERR: cannot be converted to str"));

		data::parse_into_map(&file.path()
			.expect("ERR: invalid path"));

*/	
//    }
    
}
