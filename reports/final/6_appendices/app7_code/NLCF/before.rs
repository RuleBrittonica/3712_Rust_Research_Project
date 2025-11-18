fn sum_until_negative(xs: &[i32]) -> i32 {
    let mut sum = 0;
    for &x in xs {
        // EXTRACT START
        if x < 0 { break; }   // non-local `break`
        sum += x;
        // EXTRACT END
    }
    sum
}
