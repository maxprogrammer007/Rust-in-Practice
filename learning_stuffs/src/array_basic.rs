use std::io;

fn main() {

	let a = [1,2,3,4,5];


	let first = a[0];
	let second = a[1];

	println!(" The first number of the array is {}", first);
	println!(" The second number of the array is {}", second);


	let mut index = String::new();

	io::stdin()
		.read_line(&mut index)
		.expect("Failed to read the line");


	let index: usize = index
		.trim() 
		.parse()
		.expect("Index entered was a wrong number");

	let element  = a[index];

	println!("The value of the element at index {index} is : {element}");

}