fn main(){
    println!("Hello,World!");

    // wait for three seconds
    std::thread::sleep(std::time::Duration::from_secs(3));

    println!("Hello,World! after 3 seconds");
}