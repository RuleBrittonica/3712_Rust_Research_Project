use gtk::{Button, Grid, Label, Window};
use gtk::prelude::*;
// use crate::gui::{
//     apparent_wind_accident_gui,
//     double_track_gui,
//     single_track_gui,
//     solar_incident_gui,
//     settings_gui,
// };

pub fn build_ui(app: &gtk::Application) {
    let window = Window::builder()
        .application(app)
        .title("Home Screen")
        .default_width(800)
        .default_height(600)
        .build();

    let grid = Grid::builder()
        .row_homogeneous(true)
        .column_homogeneous(true)
        .build();

    // Column Headers
    let vehicle_stability_label = Label::new(Some("Vehicle Stability"));
    let other_label = Label::new(Some("Other"));
    let configuration_label = Label::new(Some("Configuration"));

    grid.attach(&vehicle_stability_label, 0, 0, 1, 1);
    grid.attach(&other_label, 1, 0, 1, 1);
    grid.attach(&configuration_label, 2, 0, 1, 1);

    // Create and add buttons for each column
    let apparent_wind_accident_button = Button::with_label("Apparent Wind Accident");
    let double_track_button = Button::with_label("Double Track");
    let single_track_button = Button::with_label("Single Track");

    let solar_incident_angle_button = Button::with_label("Solar Incident Angle");

    let settings_button = Button::with_label("Settings");

    // Add buttons under respective column headers
    grid.attach(&apparent_wind_accident_button, 0, 1, 1, 1);
    grid.attach(&double_track_button, 0, 2, 1, 1);
    grid.attach(&single_track_button, 0, 3, 1, 1);

    grid.attach(&solar_incident_angle_button, 1, 1, 1, 1);

    grid.attach(&settings_button, 2, 1, 1, 1);

    window.set_child(Some(&grid));
    window.present();
}

