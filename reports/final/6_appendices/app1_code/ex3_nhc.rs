fn push_block(values: Vec<i32>) {
    let mut v = values;
    v.push(4);
    println!("Values are now: {:?}", v);
}

fn main() {
    let mut values = vec![1, 2, 3];
    push_block(values);
    println!("Final values: {:?}", values); // error: `values` was moved
}
