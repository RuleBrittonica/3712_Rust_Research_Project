fn main() {
    let mut values = vec![1, 2, 3];

    // Some inline block that mutates `values`:
    values.push(4);
    println!("Values are now: {:?}", values);
}
