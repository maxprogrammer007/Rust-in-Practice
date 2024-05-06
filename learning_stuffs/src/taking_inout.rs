use std::io;

fn main() {
	println!("Helo World ");
	let mut input = String::new();

	io :: stdin().read_line(&mut input).expect("Failed to read line ");

	println!("{}",input );
}