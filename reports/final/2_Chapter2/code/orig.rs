fn main() {
//Rust-Analyzer(single-file) can't
//know what `Vec` is here.
//Even if `Vec` was strong typed
    let mut v = Vec::<i32>::new();
    for i in 0..10 {
        v.push(i);
    }

    println!("{:?}", v);
}
//
//