extern crate gtk;
extern crate exif;
extern crate chrono;
extern crate regex;
extern crate walkdir;
use std::env;

use gtk::prelude::*;

use build_ui::build_ui;

mod build_ui;

fn main() {

    env::set_var("RUST_BACKTRACE", "1");
    let application =
        gtk::Application::new(Some("org.bkupeval01"), Default::default());

    application.connect_activate(build_ui);

    application.run();

}
