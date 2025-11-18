fn min_of<T: Ord + Copy>(a: T, b: T) -> T {
    choose_min(a, b)
}

fn choose_min<T: Ord + Copy>(a: T, b: T) -> T {
    if a < b { a } else { b }
}
