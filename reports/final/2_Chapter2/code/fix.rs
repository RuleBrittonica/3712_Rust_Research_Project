fn main() {
    let v = build_vec();
    println!("{:?}", v);
}

//With std in scope,
//This return type can be
//Correctly inferred
fn build_vec() -> Vec<i32> {
    let mut v = Vec::<i32>::new();
    for i in 0..10 { v.push(i); }
    v
}