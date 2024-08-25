fn main() {
let number = 3 ;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    // nested if else
    let num = 12;
    if num % 2 == 0 {
        println!("{} is even", num);
    } else {
        println!("{} is od",num);

        if num > 10 {
            println!("{} is greater than 10", num);
        } else {
            println!("{} is less than 10", num);
        }
    }
    // if conditions
    let a = 10;
    let b = 20;
    let c = 20;

    // using && to check if 'a" is less than 'b' and 'b' is equal to 'c'

    if a > b && b > c{
        println!("a is greater than b and b is greater than c");
    } else {
        println!("condition is false");
    }

    // using || to check if 'a' is less than 'b' or 'b' is equal to 'c'
    if a > b || b > c {
        println!("atleast one condition is true");
    } else {
        println!("both conditions are false");
    }

    // Match Statement specfic to Rust
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn value_in_dents(coin : Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let coin = Coin::Penny;

    println!("Value of coin : {}",value_in_dents(coin));
}
