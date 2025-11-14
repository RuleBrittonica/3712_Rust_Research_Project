pub fn bump<const N: usize>(
    arr: &mut [i32; N],
    k: usize,
    mark: &mut bool,
) where [i32; N]: Copy, {
    let mut i = 0;
    let limit = if k < N { k } else { N };
    while i < limit {
        if arr[i] == 0 {
            *mark = true;
        }
        arr[i] = arr[i] + 1;
        i += 1;
    }
}
fn main() {
    let mut a = [0, 0, 5, 9];
    let mut touched_zero = false;
    bump(&mut a, 3, &mut touched_zero);
    assert_eq!(a, [1, 1, 6, 9]);
    assert!(touched_zero);
}
