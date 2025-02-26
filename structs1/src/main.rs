
#[derive(Debug)]
struct User {
	active: bool,
	email: &str,
	username: &str,
	sign_in_count: u64,
}

fn main() {
	let user1 = User {
		active: true,
		email: "some@email.com",
		username: "somename",
		sign_in_count: 1,
	};

    println!("Hello, world!");
}
