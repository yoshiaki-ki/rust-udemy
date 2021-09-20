#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn create(width: i32, height: i32) -> Self {
        Self { width, height }
    }
    fn area(&self) {
        println!("{}", self.width * self.height)
    }
}

pub fn run() {
    let user1 = User {
        username: String::from("test-user"),
        email: String::from("aaa@test.com"),
        sign_in_count: 0,
        active: true,
    };
    let mut user1 = User {
        username: String::from("test-user"),
        email: String::from("aaa@test.com"),
        sign_in_count: 0,
        active: true,
    };
    user1.email = String::from("bbb@test.com");

    println!("{:#?}", user1);

    let user2 = build_user(String::from("test2@email.com"), String::from("user2"));
    println!("{:#?}", user2);

    let rect = Rectangle::create(20, 40);
    println!("{:#?}", rect);
    rect.area();
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
}
