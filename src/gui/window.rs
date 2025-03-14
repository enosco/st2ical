mod file_hub;

use crate::app;

use gtk4::{glib, gio,
	   prelude::*,
	   subclass::prelude::*};

use file_hub::FileChooserHub;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
	@extends gtk4::ApplicationWindow, gtk4::Window, gtk4::Widget,
    @implements gio::ActionMap, gio::ActionGroup;
}

impl Window {
    pub fn new(app: &app::App) -> Self {
	glib::Object::builder().property("application", app).build()
    }
}

mod imp {
    use super::*;
    use gtk4::CompositeTemplate;
    use glib::subclass::InitializingObject;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/st2ical/resources/window.ui")]
    pub struct Window {
	file_hub: Option<FileChooserHub>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
	const NAME: &'static str = "Window";
	type Type = super::Window;
	type ParentType = gtk4::ApplicationWindow;

	fn class_init(class: &mut Self::Class) {
	    class.bind_template();
	}

	fn instance_init(obj: &InitializingObject<Self>) {
	    obj.init_template();
	}
    }
    
    impl ObjectImpl for Window {
	fn constructed(&self) {	   
	    self.parent_constructed();
	    
	    let file_hub = FileChooserHub::new();
	    self.obj().set_child(Some(&file_hub));

	}
    }
    
    impl WidgetImpl for Window {}
    
    impl WindowImpl for Window {}

    impl ApplicationWindowImpl for Window {}
}

