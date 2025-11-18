fn main() {
    let v = build_vec();
    println!("{:?}", v);
}

//Rust-Analyzer's *generated* 
//signature uses a placeholder
//as it can't find `Vec`
fn build_vec() -> _ {
    let mut v = Vec::<i32>::new();
    for i in 0..10 { v.push(i); }
    v
}