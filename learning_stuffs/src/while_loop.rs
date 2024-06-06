let mut x = 0;
while x < 5 {
    println!("{}", x);
    if x == 3 { continue } // skipping iteration when x is equal to 3.
    x += 1;
}