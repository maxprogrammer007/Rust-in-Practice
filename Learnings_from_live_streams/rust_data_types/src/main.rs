// data types in rust
fn main() {
    // scalar types
        // - Integer
        // - Floating-point
        // - Boolean
        // - Character
    // compound types
      // - Tuple
      // - Array
    // Custom data types
      // - Struct
      // - Enum

    // Integer
    let a: u8 = 255; // 0 to 255
    let b: i8 = -128; // -128 to 127
    let c: u16 = 65535; // 0 to 65535
    let d: i16 = -32768; // -32768 to 32767
    let e: u32 = 4294967295; // 0 to 4294967295
    let f: i32 = -2147483648; // -2147483648 to 2147483647
    let _g: u64 = 18446744073709551615; // 0 to 18446744073709551615
    let _h: i64 = -9223372036854775808; // -9223372036854775808 to 9223372036854775807
    let _i: u128 = 340282366920938463463374607431768211455; // 0 to 340282366920938463463374607431768211455
    let _j: i128 = -170141183460469231731687303715884105728; // -170141183460469231731687303715884105728 to 170141183460469231731687303715884105727

    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
    println!("e: {}", e);

    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';


    println!("decimal: {}", decimal);
    println!("hex: {}", hex);
    println!("octal: {}", octal);
    println!("binary: {}", binary);
    println!("byte: {}", byte);

}