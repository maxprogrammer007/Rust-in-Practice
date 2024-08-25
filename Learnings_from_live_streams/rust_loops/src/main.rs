fn main() {
 let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let a = [1,2,3,4,5];

    for element in a.iter() {
        println!("The value is {}", element);
    }

    let s = "Hello World";
    for c in s.chars() {
        println!("{}", c);
    }
}
