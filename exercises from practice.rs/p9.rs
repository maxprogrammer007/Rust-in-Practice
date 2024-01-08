/*
fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], __);

    println!("Success!");
} */

//solution
fn main() {
    let (mut x, mut y);
    (x,y) = (3, 4);
    [x, y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [1,2]);

    println!("Success!");
} 