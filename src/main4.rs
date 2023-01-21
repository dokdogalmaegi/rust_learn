struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    let active: bool = true;
    let sign_in_count: u64 = 1;

    User {
        email,
        username,
        active,
        sign_in_count
    }
}

fn main() {
    let user = build_user(String::from("test@gmail.com"), String::from("test"));
    let mut mutable_user = build_user(String::from("test@gmail.com"), String::from("test"));

    let black : Color = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);

    println!("{}\n{}", user.email, mutable_user.email);
}