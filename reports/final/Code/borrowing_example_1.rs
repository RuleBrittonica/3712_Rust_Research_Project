fn add_exclamation(message: &mut String) {
    message.push('!');
}

fn main() {
    let mut greeting = String::from("Hello");
    add_exclamation(&mut greeting); // Borrows greeting mutably
    println!("{}", greeting);       // Prints "Hello!"
}