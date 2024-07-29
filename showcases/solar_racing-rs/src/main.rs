use gdk::Display;
use gtk::{Application, CssProvider};
use gtk::prelude::*;

mod gui;
mod utils;

const APP_ID: &str = "com.solar_racing";

fn main() {
    // Create the application
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    // Connect to startup signal to load CSS
    app.connect_startup(|_| {
        load_css();
    });

    // Connect to activate signal
    app.connect_activate(|app| {
        gui::gui_main::build_ui(app);
    });

    // Run the application
    app.run();
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
