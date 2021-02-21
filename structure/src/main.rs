#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    println!("Hello, world!");
    let user = User {
        username: String::from("Jeffrey Lomibao"),
        email: String::from("Jeffrey.Lomibao@georgfischer.com"),
        sign_in_count: 0,
        active: false,
    };
    println!("{:?}", user);
}
