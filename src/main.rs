extern crate gtk;
extern crate exif;
extern crate chrono;
extern crate regex;
extern crate walkdir;
use std::env::args;

use gtk::prelude::*;

use build_ui::build_ui;

mod build_ui;

fn main() {

    let application =
        gtk::Application::new(Some("org.bkupeval01"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());

}
