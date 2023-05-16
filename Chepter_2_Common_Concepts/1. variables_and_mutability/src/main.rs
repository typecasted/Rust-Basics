fn main() {
    // a simple immutable variable
    let x = 10;

    println!("immutable x = {x}");

    mutable_variable();

    shadow_variable();
}

/// [mutable_variable] function will show the use mutable variable
fn mutable_variable() {
    // a simple mutable variable
    let mut mut_x = 5;

    println!("mut_x = {mut_x} ");

    mut_x = 10;

    println!("mut_x after changes = {mut_x}");
}

/// [shadow_variable] function will demonstrate the use of shadowing.
fn shadow_variable() {
    // here shadow_var variable which is immutable by default.
    let shadow_var = 10;

    println!("before shadowing : {shadow_var}");

    // shadowing only happen if use the "let" keyword ahead. otherwise it will prompt the error.
    let shadow_var = "Hello, World";

    println!("after shadowing : {shadow_var}");
}
