// A const function that can be evaluated at compile time
const PREFIX: u32 = 10;
const fn compute_total(a: u32, b: u32) -> u32 {
    compute_total_inner(a, b)
}

// Extracted function still marked as const
const fn compute_total_inner(a: u32, b: u32) -> u32 {
    PREFIX + a + b
}