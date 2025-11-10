use std::fmt::Display;

fn show_twice(d: &dyn Display) -> String {
    // EXTRACT START
    format!("{} | {}", d, d)
    // EXTRACT END
}
