use std::fmt::Display;

fn show_twice(d: &dyn Display) -> String {
    show_twice_inner(d)
}

fn show_twice_inner(d: &dyn Display) -> String {
    format!("{} | {}", d, d)
}
