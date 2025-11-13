fn main() {
    let mut x = 0;
    mutate_x(&mut x);
    assert(x == 3);
}

fn mutate_x(x: &mut i32) {
    *x = 3;
}