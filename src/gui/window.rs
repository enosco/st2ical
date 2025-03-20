pub mod file_hub;

use crate::app;

use gtk4::{glib, gio,
	   prelude::*,
	   subclass::prelude::*};

use file_hub::FileChooserHub;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
	@extends gtk4::ApplicationWindow, gtk4::Window, gtk4::Widget,
    @implements gtk4::Buildable; // gio::ActionMap, gio::ActionGroup;
}

impl Window {
    pub fn new(app: &app::App) -> Self {
	glib::Object::builder()
	    .property("application", app)
	    .build()
    }

    pub fn file_hub(&self) -> FileChooserHub {
	self.imp().file_hub.get()
    }

    pub fn file_view(&self) -> gtk4::ColumnView {
	self.imp().file_view.get()
    }
}

mod imp {
    use super::*;
    use gtk4::CompositeTemplate;
    use glib::subclass::InitializingObject;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/st2ical/resources/window.ui")]   
    pub struct Window {
	#[template_child]
	pub file_hub: TemplateChild::<FileChooserHub>,
	#[template_child]
	pub file_view: TemplateChild::<gtk4::ColumnView>,
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

	    let file_hub = self.obj().file_hub();
	}
    }
    
    impl WidgetImpl for Window {}
    
    impl WindowImpl for Window {}

    impl ApplicationWindowImpl for Window {}
}

