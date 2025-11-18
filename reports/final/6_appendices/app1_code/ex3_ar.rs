fn push_block(values: &mut Vec<i32>) {
    values.push(4);
    println!("Values are now: {:?}", values);
}

fn main() {
    let mut values = vec![1, 2, 3];
    push_block(&mut values);
    println!("Final values: {:?}", values); // Now values is still in scope and updated
}
