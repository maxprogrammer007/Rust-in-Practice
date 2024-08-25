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
}
