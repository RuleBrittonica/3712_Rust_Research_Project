fn hoisted_block(x: i32) {
    let y = x * 2;
    println!("y is {}", y);
}

fn main() {
    let x = 42;
    hoisted_block(x);
}
