
#[derive(Debug)]
struct _User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn _struct_test() {
    let user1 = _User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("user is {:?}", user1);

    let user2 = _User{
        email: String::from("another@example.com"),
        ..user1
    };

    println!("user2.username is {:?}", user2.username);
}