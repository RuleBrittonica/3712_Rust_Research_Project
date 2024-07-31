use gtk::prelude::*;
use gtk::{Button, Grid, Label, Window};
use std::cell::RefCell;
use std::rc::Rc;

pub fn create_settings_screen(app: &gtk::Application, path: String) {
    let window = Window::builder()
        .application(app)
        .title(&path)
        .default_width(400)
        .default_height(200)
        .build();

    let window_rc = Rc::new(RefCell::new(window));

    let grid = Grid::builder()
        .row_homogeneous(true)
        .column_homogeneous(true)
        .build();

    let hello_label = Label::new(Some("Settings Screen"));
    let back_button = Button::with_label("Back");

    let window_clone = window_rc.clone();
    back_button.connect_clicked(move |_| {
        window_clone.borrow().close();
    });

    grid.attach(&hello_label, 0, 0, 1, 1);
    grid.attach(&back_button, 0, 1, 1, 1);

    window_rc.borrow().set_child(Some(&grid));
    window_rc.borrow().present();
}
