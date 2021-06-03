
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

fn main() {
    let user1 = User{
        username: String::from("testuser"),
        email: String::from("useremail@email.test"),
        sign_in_count: 1,
        active: true,
    };

    println!("{} {} {} {}", user1.email, user1.username, user1.sign_in_count, user1.active);

    let user2 = User{
        username: String::from("testuser2"),
        ..user1
    };

    println!("{} {} {} {}", user2.email, user2.username, user2.sign_in_count, user2.active);

    let color1 = Color(1,2,3);

    println!("{}", color1.2);
}
