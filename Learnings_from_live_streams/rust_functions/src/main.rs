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

// statements and expressions
// Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value. Let's see an example of both:

fn statements_and_expressions() {
    let y = 6; // statement
    let x = { // block is an expression
        let y = 3;
        y + 1 // expression
    };
    println!("The value of x is: {}", x);
}

// Function with return value

fn five() -> i32 {
    5
}
