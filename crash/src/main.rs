fn main() {
    println!("Hello, crash!");

	let tup: (u8, char, f32) = (90, 'k', 10.0);
	let tup1 = (8, 4);

	println!("tupe: {:?}.", tup);

	let mut arr: [i64; 10];
	arr = [890; 10];

	println!("array: {:?}", arr);

	println!("length of array: {}", arr.len());

}
