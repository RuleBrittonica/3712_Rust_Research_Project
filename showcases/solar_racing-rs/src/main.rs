use gtk::Application;
use gtk::prelude::*;

mod gui;
mod utils;

const APP_ID: &str = "com.solar_racing"; // Ensure this is valid

fn main() {
    // Create the application
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    // Connect to activate signal
    app.connect_activate(|app| {
        gui::gui_main::build_ui(app);
    });

    // Run the application
    app.run();
}
