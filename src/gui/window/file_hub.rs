use std::cell::RefCell;
use std::cell::Cell;
use std::sync::OnceLock;
use gtk4::{glib, gio, prelude::*, subclass::prelude::*};
use glib::clone;

glib::wrapper! {
    pub struct FileChooserHub(ObjectSubclass<imp::FileChooserHub>)
	@extends gtk4::Widget;
}

impl FileChooserHub {
    pub fn new() -> Self {
	glib::Object::new()
    }

    pub fn file(&self) -> Option<gio::File> {
	self.imp().file.borrow().clone()
    }
    
    fn setup_callbacks(&self) {
	let imp = self.imp();

	imp.file_button.connect_clicked(clone!(
	    #[weak] imp,
	    move |_| {
		imp.obj().open_dialog();
	    })
	);
	
	imp.load_button.connect_clicked(clone!(
	    #[weak] imp,
	    move |_| {
		imp.obj().emit_by_name::<()>("load-button-clicked", &[]);
	    })
        );    
    }

    fn open_dialog(&self) {
	let dialog = gtk4::FileDialog::builder()
	    .accept_label("Open")
	    .modal(true)
	    .build();

	// getting the parent in this way might be limiting.
	// maybe require a parent window on creation like how
	// a window requires an app?
	let parent_window = self.root()
	    .unwrap()
	    .dynamic_cast::<gtk4::Window>();
	 
	if let Ok(parent) = parent_window {
	    dialog.open(Some(&parent), gio::Cancellable::NONE, clone!(
		#[weak(rename_to = imp)] self.imp(),
		move |file| {
		    if let Ok(file) = file {					
			imp.file_label.set_label(
			    file.basename()
				.expect("ERR: invalid pathbuf")
				.to_str()
				.expect("ERR: cannot be converted to str"));
			
			*imp.file.borrow_mut() = Some(file);
		    }		
		})
	    );	    
	}
    }
 

}

mod imp {
    use super::*;
    use gtk4::CompositeTemplate;
    use glib::subclass::{InitializingObject, Signal};

    
    #[derive( CompositeTemplate, Default)]
    #[template(resource = "/st2ical/resources/file_hub.ui")]   
//    #[derive(Default)]
    pub struct FileChooserHub {
	pub file: RefCell<Option<gio::File>>,
	    
	#[template_child]
	pub file_button: TemplateChild<gtk4::Button>,
	#[template_child]
	pub file_label: TemplateChild<gtk4::Label>,
	#[template_child]
	pub load_button: TemplateChild<gtk4::Button>,	
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FileChooserHub {
	const NAME: &'static str = "FileChooserHub";
	type Type = super::FileChooserHub;
	type ParentType = gtk4::Widget;

	fn class_init(class: &mut Self::Class) {	    
	    class.bind_template();
	    class.set_layout_manager_type::<gtk4::BinLayout>();
	}

	fn instance_init(obj: &InitializingObject<Self>) {
	    obj.init_template();
	}
    }

    impl ObjectImpl for FileChooserHub {
	fn constructed(&self) {	   
	    self.parent_constructed();
	    self.obj().setup_callbacks();
	}	    
	
	fn signals() -> &'static [Signal] {
	    static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
	    SIGNALS.get_or_init(|| {
		vec![Signal::builder("load-button-clicked")
                     .build()]		   
            })
	}
	
	fn dispose(&self) {
	    while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
	}
    }
    
    impl WidgetImpl for FileChooserHub {}
    
}
