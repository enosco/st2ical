mod app;
mod gui;

use app::App;
use gtk4::{glib, gio, prelude::*};

fn main() -> glib::ExitCode {

    gio::resources_register_include!("st2ical.gresource")
	.expect("ERR: failed to register resources.");

    let app = App::new("com.github.st2ical");
    
    
    
    app.run()
}
