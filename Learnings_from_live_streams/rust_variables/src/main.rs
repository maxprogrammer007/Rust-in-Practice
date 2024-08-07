fn main() {
    let x = 5;
    println!("The value of x is :{}", x);
    // x = 6; // This will give an error because x is immutable
    // shadowing - we can declare a new variable with the same name as the previous one
    {
        let x = 6;
        println!("The value of x is :{}", x);
    }

    let mut y = x;
    y = y + 4;
    println!("The value of y is : {} ", y);

    // const variables
    const MAX_POINTS: u32 = 100_000; // _ can be used to make the number more readable
    println!("The value of MAX_POINTS is : {}", MAX_POINTS);
}