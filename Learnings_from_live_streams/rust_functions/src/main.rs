// Functions -  Functions are the building blocks of a Rust program. Functions are used to break the code into smaller and modular pieces. Functions are defined using the fn keyword. The syntax of a function is as follows:

 fn main() {
    println!("Hello, world!");
    another_function();
    another_function1(5);
    statements_and_expressions();
    let x = five();
    println!("The value of x is: {}", x);
    let result = sum(5, 10);
    println!("The sum of 5 and 10 is: {}", result);
    let k = sum_diff(23,98);
    println!("The sum and difference of the given numbmer is {} and {}",k.0,k.1);
 }

fn sum(num1 :i32,num2:i32) ->i32{
    num1+num2
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
    5 + 1 // expression

}

//multiple return values

fn sum_diff(num1: i32, num2: i32)->(i32,i32){
    (num1+num2,num1-num2)
}