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
    println!("Hello, world!");
}
