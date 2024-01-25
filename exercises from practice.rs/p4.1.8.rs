/* 
fn main() {
    assert!(0.1+0.2==0.3);

    println!("Success!");
} */


fn main() {
	// First method
    assert!(0.1_f32+0.2_f32==0.3);

    println!("Success!");
    // Second Method

    assert!(0.1 as f32+0.2 as f32==0.3);

    println!("Success!");

}
