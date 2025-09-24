// Original
fn foo() {
    let mut x = 0i32;
    x = x + 1;
    assert!(x == 1);
}
// Refactored
fn foo() {
    let mut x = 0i32;
    x = incr(x);
    assert!(x == 1);
}
fn incr(x: i32) -> i32 {
    x + 1
}