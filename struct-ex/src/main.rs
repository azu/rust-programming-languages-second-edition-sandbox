struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user = build_user(
        String::from("test"),
        String::from("name"),
    );
    let updated_user = update_email(user, String::from("mail@example.com"));
    println!("{}", updated_user.email);


    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{} ", black.0)
}

fn build_user(email: String, username: String) -> User {
    return User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };
}

fn update_email(user: User, email: String) -> User {
    // 構造体更新記法
    return User {
        email,
        ..user
    };
}