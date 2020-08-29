fn main() {
    let user = new_user(String::from("andrew@gmail.com"), String::from("andrew"));
    let user2 = User {
        email: String::from("new@new.com"),
        username: String::from("new"),
        ..user
    };
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rectangle = Rectangle {
        width: 10,
        height: 23,
    };
    println!("{}", rectangle.area());
    println!("{:#?}", rectangle);
}

fn new_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}