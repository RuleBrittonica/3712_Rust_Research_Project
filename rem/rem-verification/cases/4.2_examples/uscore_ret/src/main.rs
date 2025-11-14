fn local_inference() {
    let x: _ = 42;
    let y = x + 1;
    assert!(y == 43);
}

fn main() {
    local_inference();
}