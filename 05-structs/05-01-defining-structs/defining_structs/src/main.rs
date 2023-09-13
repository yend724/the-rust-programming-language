fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    let user = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        active: true,
        sign_in_count: 1,
    };
    println!("user.name = {}", user.username);
    println!("user.email = {}", user.email);
    println!("user.active = {}", user.active);
    println!("user.sign_in_count = {}", user.sign_in_count);

    let user1 = build_user(
        String::from("someusername123"),
        String::from("someone@example.com"),
    );
    println!("user1.name = {}", user1.username);
    println!("user1.email = {}", user1.email);
    println!("user1.active = {}", user1.active);
    println!("user1.sign_in_count = {}", user1.sign_in_count);
    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    let _user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let _user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}
