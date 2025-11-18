pub fn moving_average_at(data: &[i32], i: usize) -> i32 {
    let n = data.len();

    let mut sum: i32 = 0;
    let mut count: i32 = 0;

    if i > 0 {
        sum += data[i - 1];
        count += 1;
    }

    sum += data[i];
    count += 1;

    let ip1 = i + 1;
    if ip1 < n {
        sum += data[ip1];
        count += 1;
    }

    sum / count
}

pub fn moving_average_3(data: &[i32], out: &mut [i32]) {
    let n = data.len();
    let m = out.len();

    if n == 0 || m == 0 {
        return;
    }

    let mut i: usize = 0;
    while i < n && i < m {
        out[i] = moving_average_at(data, i);
        i += 1;
    }
}

pub fn run_example() {
    let data = [1, 3, 5, 7];
    let mut out = [0; 4];

    moving_average_3(&data, &mut out);

    // Manually:
    // i=0: avg(1,3) = 2
    // i=1: avg(1,3,5) = 3
    // i=2: avg(3,5,7) = 5
    // i=3: avg(5,7) = 6
    assert!(out[0] == 2);
    assert!(out[1] == 3);
    assert!(out[2] == 5);
    assert!(out[3] == 6);
}
