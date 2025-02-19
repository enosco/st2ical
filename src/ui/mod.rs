mod imp;

use gtk4 as gtk;
use gtk::glib::{self, clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::Application;//, FileChooserNative, FileChooserAction};


// Establish custom class, UI, based on application window
glib::wrapper! {
    pub struct UI(ObjectSubclass<imp::UI>)
	@extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
	@implements gtk::Accessible, gtk::Buildable, gtk::Native;
}

// Public interface
impl UI {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn connect_widgets(&self) {
	let ui = self.imp();

	ui.file_button.connect_clicked(clone!(
	    #[weak] ui,
	    move |_| {
		Self::load_file(ui.obj().as_ref());
	}));
	
	
    }

    fn load_file(parent: &UI) {
	let file_chooser = gtk::FileChooserNative::builder()
	    .action(gtk::FileChooserAction::Open)
	    .modal(true)
	    .transient_for(parent)
	    .build();
	
	file_chooser.show();
    }
  
}
