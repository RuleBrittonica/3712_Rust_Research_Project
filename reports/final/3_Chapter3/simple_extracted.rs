fn sum_and_print(x: i32, y: i32) {
    let total = add(x, y);
    println!("The total is {}", total);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}