
# Data types
There are two types of data types in Rust.
    1. Scaler types
    2. Compound types


## 1. Scaler types
### i. Integer types
        - 8 bit     :   i8      |       u8
        - 16 bit    :   i16     |       u16
        - 32 bit    :   i32     |       u32
        - 64 bit    :   i64     |       u64
        - 128 bit   :   i128    |       u128
        - arch      :   isize   |       usize

### ii. Floating Point types
        - f32
        - f64

### iii. Boolean type

    iv. Character type

## 2. Compound Types

### i. Tuple type
        example:
        let tup: (i32, f64, u8) = (500, 6.4, 1);

### ii. Array Type
        example:
        let a = [1, 2, 3, 4, 5];
        let a: [i32; 5] = [1, 2, 3, 4, 5];

        let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];