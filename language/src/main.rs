const HOURS_A_DAY: u8 = 24;

fn main() {
    println!("Hello, world!");

	let mut spaces = "abcde";

	println!("{}, {}, {}.", HOURS_A_DAY, spaces, spaces);

	//let	 spaces = spaces.len();
	spaces = "bnnnmbn";

	println!("{}, {}, {}.", HOURS_A_DAY, spaces, spaces);

	let guess: u32 = "xxxx".parse().expect("Not a number");

	let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
"August", "September", "October", "November", "December"];
}
