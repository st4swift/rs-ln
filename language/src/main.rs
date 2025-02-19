//use std::io;

//const HOURS_A_DAY: u8 = 24;

fn main() {
	let mut s: &str = "kkkkkk";
	println!("{s}");
	println!("{:?}", s);

	let mut s = String::from("LLLOOOOOOLLL"); 
	
	println!("{s}");
	
	s.push_str(", Hello");

	println!("{s}");

/*	for n in (1..8).rev() { println!("{n}"); }

	println!("LIFTOFF");

	let mut counter = 0;

	let result = loop { 
		counter +=  1;
		if counter == 10 {
			break counter * 3;
		}
	};

	println!("the result is {result}!");

	let condition = false;
	let number = if condition {5} else {9};


	println!("number={number}");*/
}

	/*
	let a = [1, 2, 3, 4, 5];
	println!("please enter an array index:");
	let mut index = String::new();
	io::stdin().read_line(&mut index).expect("Failed to read line.");
	let index: usize = index.trim().parse().expect("Index entered was not a number.");
	let element = a[index];

	println!("the value of the element at index {index} is: {element}.");

   println!("Hello, world!");

	let mut spaces = "abcde";

	println!("{}, {}, {}.", HOURS_A_DAY, spaces, spaces);

	//let	 spaces = spaces.len();
	spaces = "bnnnmbn";

	println!("{}, {}, {}.", HOURS_A_DAY, spaces, spaces);

	let guess: u32 = "xxxx".parse().expect("Not a number");

	let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
"August", "September", "October", "November", "December"];
*/




