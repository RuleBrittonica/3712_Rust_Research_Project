fn bump_extracted<const N: usize>(
    mut arr: [i32; N], i: usize, mut mark: bool
) where  [i32; N]: Copy, {
    if arr[i] == 0 {
        mark = true;
    }
    arr[i] = arr[i] + 1;
}
pub fn bump<const N: usize>(
    arr: &mut [i32; N],
    k: usize,
    mark: &mut bool,
) where [i32; N]: Copy, {
    let mut i = 0;
    let limit = if k < N { k } else { N };
    while i < limit {
        bump_extracted::<N>((*arr).clone(), i, *mark);
        i += 1;
    }
}
fn main() {
    let mut a = [0, 0, 5, 9];
    let mut touched_zero = false;
    bump(&mut a, 3, &mut touched_zero);
    assert_eq!(a, [1, 1, 6, 9]); // fails: a: [0, 0, 5, 9]
    assert!(touched_zero); // fails
}