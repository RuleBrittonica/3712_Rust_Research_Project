// Pre-Extraction
fn foo() -> u32 {
    let n = 2; let m = 1;
    n + m
}
// After Extraction
fn new_foo() -> u32 {
    let n = 2;
    extracted_fn(n)
}
fn extracted_fn(n: u32) -> u32 {
    let m = 1;
    n + m
}