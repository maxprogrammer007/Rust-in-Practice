// data types in rust

enum IpAddrKind {
    V4,
    V6,
}
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
    let _f: i32 = -2147483648; // -2147483648 to 2147483647
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


    // Floating Points

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("x: {}", x);
    println!("y: {}", y);

    // numeric operations
    let sum = x + y;
    let difference = x - y;
    let product = x * y;
    let quotient = x / y;
    let remainder = x % y;

    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("remainder: {}", remainder);

    // Boolean types

    let t = true; // implicit type
    let f: bool = false; // explicit type

    println!("t: {}", t);
    println!("f: {}", f);

    // if
    if t {
        println!("t is true");
    } else {
        println!("t is false");
    }

    let not_t = !t;
    let and = t && f;
    let or = t || f;

    println!("not_t: {}", not_t);
    println!("and: {}", and);
    println!("or: {}", or);

    // Character type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c: {}", c);
    println!("z: {}", z);
    println!("heart_eyed_cat: {}", heart_eyed_cat);

    // iterate over characters in a string
    for c in "hello".chars() {
        println!("{}", c);
    }

    // Tuple
    let tup: (i32,f64,u8) = (500, 6.4 ,1);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    // Accessing by index

    for i in 0..3 {
        println!("{}: {}", i, tup.0);
    }

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("five_hundred: {}", five_hundred);
    println!("six_point_four: {}", six_point_four);
    println!(" one : {}", one);

    // Arrays

    let arr = [1,2,3,4,5,6];
    let first = arr[0];
    let second = arr[1];
    let third = arr[2];
    let fourth = arr[3];
    let fifth = arr[4];
    let sixth = arr[5];

    println!("first: {}", first);
    println!("second: {}", second);
    println!("third: {}", third);
    println!("fourth: {}", fourth);
    println!("fifth: {}", fifth);
    println!("sixth: {}", sixth);

    // Accessing by index
    for i in 0..6 {
        println!("{}: {}", i, arr[i]);
    }

    for element in arr.iter() {
        println!("{}", element);
    }

    // Custom data types
    // Struct
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("someone@gmail.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1: {}", user1.username);

    // Accessing struct fields
    println!("user1 email: {}", user1.email);
    println!("user1 sign_in_count: {}", user1.sign_in_count);
    println!("user1 active: {}", user1.active);

    // Update struct fields
    let _user2 = User {
        email: String::from("okay@gmail.com"),
        username: String::from("okay"),
        active: true,
        sign_in_count: 1,
    };

    // Enum
    let _home = IpAddrKind::V4;
    let _loopback = IpAddrKind::V6;

    // Enum with data
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let _home = IpAddr::V4(String::from("12344568"));
    let _loopback = IpAddr::V6(String::from("12344568"));

    // Enum with different types of data

    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let _home = IpAddr2::V4(127, 0, 0, 1);
    let _loopback = IpAddr2::V6(String::from("::1"));



}