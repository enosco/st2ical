mod ui;
mod data;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{gio, glib, Application};
use ui::UI;

fn main() -> gtk4::glib::ExitCode {

    gio::resources_register_include!("st2ical.gresource")
	.expect("Failed to register resources.");
    
    let app = gtk4::Application::builder()
	.application_id("com.github.st2ical")
	.build();

    app.connect_activate(init);
    app.run()
}

fn init(app: &Application) {
    let app_ui = UI::new(app);
    app_ui.present();
}


   
