fn main() {
    let mut x = 0;
    mutate_x(x.clone());
    assert(x == 3);
}

fn mutate_x(mut x: i32) {
    x = 3;
}