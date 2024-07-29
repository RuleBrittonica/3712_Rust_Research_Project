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

    let vehicle_stability_col: i32 = 0;
    let other_col: i32 = 1;
    let configuration_col: i32 = 2;

    grid.attach(&vehicle_stability_label, vehicle_stability_col, 0, 1, 1);
    grid.attach(&other_label, other_col, 0, 1, 1);
    grid.attach(&configuration_label, configuration_col, 0, 1, 1);

    // Create and add buttons for each column
    let apparent_wind_accident_button = Button::with_label("Apparent Wind Accident");
    let double_track_button = Button::with_label("Double Track");
    let single_track_button = Button::with_label("Single Track");

    let solar_incident_angle_button = Button::with_label("Solar Incident Angle");

    let settings_button = Button::with_label("Settings");

    // Add buttons under respective column headers
    grid.attach(&apparent_wind_accident_button, vehicle_stability_col, 1, 1, 1);
    grid.attach(&double_track_button, vehicle_stability_col, 2, 1, 1);
    grid.attach(&single_track_button, vehicle_stability_col, 3, 1, 1);

    grid.attach(&solar_incident_angle_button, other_col, 1, 1, 1);

    grid.attach(&settings_button, configuration_col, 1, 1, 1);

    window.set_child(Some(&grid));
    window.present();
}

