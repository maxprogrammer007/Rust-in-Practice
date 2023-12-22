fn main() {
	let tup: (i32,f64,u8) = ( 500, 76.88 , 2);
	let (x,y,z) = tup;

	println!("The value of x in tuple is : {}",x);
	println!("The value of y in tuple is : {}",y);
	println!("The value of z in tuple is : {}",z);
}
