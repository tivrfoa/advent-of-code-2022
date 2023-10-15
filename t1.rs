fn main() {
	println!("{}", i32::MAX);
	println!("10000000");
	println!("{}", i32::MAX > 10000000);

	let mut s1 = "hello".to_string();
	s1.pop();
	println!("{s1}");
}
