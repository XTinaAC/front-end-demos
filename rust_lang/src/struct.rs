struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("xtina@gmail.com"),
        username: String::from("xtina"),
        active: true,
        sign_in_count: 0,
    };
}
