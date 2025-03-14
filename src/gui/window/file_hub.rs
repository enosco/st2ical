use std::cell::RefCell;
use gtk4::{glib, gio, prelude::*, subclass::prelude::*};

glib::wrapper! {
    pub struct FileChooserHub(ObjectSubclass<imp::FileChooserHub>)
	@extends gtk4::Widget;
}

impl FileChooserHub {
    pub fn new() -> Self {
	glib::Object::new()
    }
}

mod imp {
    use super::*;
    #[derive(Default)]
    pub struct FileChooserHub {
	pub file: RefCell<Option<gio::File>>,
	
	file_button: RefCell<gtk4::Button>,
//	load_button: RefCell<gtk4::Button>,
//	file_label: RefCell<gtk4::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FileChooserHub {
	const NAME: &'static str = "FileChooserHub";
	type Type = super::FileChooserHub;
	type ParentType = gtk4::Widget;

	fn class_init(class: &mut Self::Class) {
	    //	    class.bind_template();
	    class.set_layout_manager_type::<gtk4::BinLayout>();
	}	
    }

    impl ObjectImpl for FileChooserHub {
	fn constructed(&self) {	   
	    self.parent_constructed();

	    let file_button = gtk4::Button::builder()
		.label("Choose File")
		.build();

	    file_button.set_parent(&*self.obj());
	    *self.file_button.borrow_mut() = file_button;
	}

	fn dispose(&self) {
	    let file_button = self.file_button.borrow_mut();
	    file_button.unparent();
	}
    }

    impl WidgetImpl for FileChooserHub {}

    
}
