fn main() {
    println!("Data Types in Rust");

    scaler_data_types();
}

// ! ---------------- Scaler data types ---------------- //
/// [scaler_data_types] will demonstrate all the scaler data types
fn scaler_data_types() {
    integer_types();
}

fn integer_types() {
    // * by default an integer will be assigned as i32 type integer.
    // * meaning 32 bit signed integer which can contain value ranging from -2147483648 to 2147483647
    let x = -2147483648;
    let lowest_i32_value: i32 = -2147483648;
    let highest_i32_value: i32 = 2147483647;

    println!("i32 x = {x}");
    println!("lowest_i32_value = {lowest_i32_value}");
    println!("highest_i32_value = {highest_i32_value}");

    // * we can also create value for 8 bit, 16 bit, 128 bit integer value; signed as well as unsigned.
}
