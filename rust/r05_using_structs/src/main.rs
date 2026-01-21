struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(String::from("waza@gmail.com"), String::from("waza"));
    println!("The user {0} who is {1} has {2} and {3} ", user2.username, user2.active, user2.email, user2.sign_in_count);

    let user3 = User {
        username: user2.username,
        active: user2.active,
        email: String::from("newemail@gmail.com"),
        sign_in_count: user2.sign_in_count,
    };

    let _user3 = User {
        email: String::from("supernewemail@gmail.com"),
        ..user3
    };

    let rect1 = Rectangle{
        width: 30,
        height: 80
    };

    let _square1 = Rectangle::square(20);

    println!("The are of the rectangle is {} square pixels", area(&rect1));
    println!("The are of the rectangle is {} square pixels", rect1.area());
    println!("The rectangle has nonzero width; it is {}", rect1.width());
    println!("The dimensions of rect1 is {rect1:#?}");
    dbg!(&rect1);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1
    }
}

fn area( rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
