// using the loop keyword

fn main(){
	let mut x = 1;
	// continue looping until x > 5
	loop{
		println!("x is {:?}",x );
		x += 1;
		if x > 5 {
			break;
		}
	}
} 