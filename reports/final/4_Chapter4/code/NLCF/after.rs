use std::ops::ControlFlow;
use std::ops::ControlFlow::{Break, Continue};

fn sum_until_negative(xs: &[i32]) -> i32 {
    let mut sum = 0;
    for &x in xs {
        match step_sum_cf(x) {
            Break(()) => break,
            Continue(delta) => sum += delta,
        }
    }
    sum
}

fn step_sum_cf(x: i32) -> ControlFlow<(), i32> {
    if x < 0 {
        Break(())
    } else {
        Continue(x)
    }
}
