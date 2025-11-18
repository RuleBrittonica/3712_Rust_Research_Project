fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // Ownership of the string data moves to s2
    // The following line would be invalid if uncommented:
    // println!("{}", s1);
    // ^ value borrowed here after move
    println!("{}", s2); // Prints "Hello"
}
