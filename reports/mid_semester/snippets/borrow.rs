let x = 5; //x is an imm variable
let x_ref = &x; //imm reference to x
let mut y = 5; //z is a mut variable
let ref_y = &mut z; //mut reference to z
//Functions can take references
fn foo(s: &String){println!("{}", s);}