// Functions -  Functions are the building blocks of a Rust program. Functions are used to break the code into smaller and modular pieces. Functions are defined using the fn keyword. The syntax of a function is as follows:

 fn main() {
    println!("Hello, world!");
    another_function();
    another_function1(5);
}

fn another_function() {
    println!("Another function.");
}

fn another_function1(number: i32) {
    println!("The value of number is : {}", number);
}