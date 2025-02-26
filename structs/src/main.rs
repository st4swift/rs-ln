#[derive(Debug)]
struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

fn build_user(email: String, username: String)-> User{
	User{
		active: true,
		email,
		username,
		sign_in_count: 1,
	}
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);


fn main() {
	let mut user1 = build_user(String::from("mail@example.com"), String::from("someone"));
	println!("{:?}", user1);

	user1.sign_in_count = 2;

	println!("{:?}", user1);

	let user2 = User {
		username: String::from("user2-name"),
		..user1
	};

	println!("{:?}", user2);
	println!("user1.username:{}", user1.username);

    println!("Hello, world!");

	let white = Color(255, 255, 255);
	let origin = Point(0, 0, 0);

	println!("Color: {:?}, Point: {:?}.", white, origin);

	let Point(x, y, z) = origin;

	println!("x: {x}, y: {y}, z: {z}.");



}
