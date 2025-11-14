fn take_inferred(x: _) -> i32 {
    // Type of x will be inferred from the call site.
    x + 1
}

fn main() {
    let result = take_inferred(5);
    assert!(result == 6);
}
