use gtk4 as gtk;

use gtk::{glib, CompositeTemplate};
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{Button, Entry, Label, TextView};

use crate::data;
use std::cell::RefCell;
use std::collections::HashMap;
/* TODO:
 * Split boxes into their own widgets
 */

// UI Object containing all widgets from composite template
#[derive(CompositeTemplate, Default)]
#[template(resource = "/st2ical/resources/window.ui")]
pub struct UI {
    /***** WIDGETS *****/
    
    /* file loading */
    #[template_child]
    pub file_button: TemplateChild<Button>,
    #[template_child]
    pub file_label: TemplateChild<Label>,

    /* course retrieval */
    #[template_child]
    pub crn_entry: TemplateChild<Entry>,

    /* course info fields */
    #[template_child]
    pub name_field: TemplateChild<Entry>,
    #[template_child]
    pub location_field: TemplateChild<Entry>,
    #[template_child]
    pub days_field: TemplateChild<Entry>,
    #[template_child]
    pub meet_start_field: TemplateChild<Entry>,
    #[template_child]
    pub meet_end_field: TemplateChild<Entry>,
    #[template_child]
    pub date_start_field: TemplateChild<Entry>,
    #[template_child]
    pub date_end_field: TemplateChild<Entry>,
    #[template_child]
    pub add_button: TemplateChild<Button>,
    
    /* file saving */
    #[template_child]
    pub selected_view: TemplateChild<TextView>,
    
    #[template_child]
    pub save_button: TemplateChild<Button>,

    /***** DATA *****/
    //    pub course_data: Cell<HashMap<i32, data::Event>>,
    pub database: RefCell<data::Database>,
//	pub selected: Cell 
}

// define UI as subclass of ApplicationWindow
#[glib::object_subclass]
impl ObjectSubclass for UI {
    const NAME: &'static str = "UserInterface"; // same as "class" in template
    type Type = super::UI;
    type ParentType = gtk::ApplicationWindow;

    // next two functions are necessary to
    // recognize the composite template
    fn class_init(class: &mut Self::Class) {
	class.bind_template();
    }
    
    fn instance_init(obj: &InitializingObject<Self>) {
	obj.init_template();
    }
}

// define traits for all GObjects
impl ObjectImpl for UI {
    fn constructed(&self) {
	self.parent_constructed();
	self.obj().setup_callbacks();
    }
}

// define traits for widgets
impl WidgetImpl for UI {}

// define traits for windows
impl WindowImpl for UI {}


// define traits for applicaiton windows
impl ApplicationWindowImpl for UI {}
