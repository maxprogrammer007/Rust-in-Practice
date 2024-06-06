fn main() {
	let mut height = 190;
	height = height - 20;

	let result = if height > 180 {
		"Tall"
	}
	else if height > 170 {
		"Average"

	}
	else {
		"Short"
	};

	println!("Result : {} ", result);

}