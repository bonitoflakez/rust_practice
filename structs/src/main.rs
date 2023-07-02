#![allow(unused)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

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

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    {
        let user1 = build_user(
            String::from("someone@example.com"),
            String::from("someuser123"),
        );

        // let user1 = User {
        //     active: true,
        //     username: String::from("someuser123"),
        //     email: String::from("someone@example.com"),
        //     sign_in_count: 1,
        // };
    }

    {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }

    {
        let subject = AlwaysEqual;
    }

    // example program using structs
    {
        let width1 = 30;
        let height1 = 50;

        println!(
            "are of rectangle is {} square pixels.",
            area(width1, height1)
        )
    }

    {
        let rect1 = (30, 50);

        println!("area of rectangle is {} square pixels.", new_ar(rect1))
    }

    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!("area of rectangle is {} square pixels.", new_ar_2(&rect1))
    }

    {
        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };

        // println!("rect1 is {:?}", rect1);
        dbg!(&rect1);
    }

    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        if rect1.width() {
            println!("rectangle has a nonzero width; it is {}", rect1.width);
        }

        println!("area of rectangle is {} square pixels.", rect1.area())
    }

    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };

        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };

        println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }
}

fn new_ar_2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn new_ar(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }

    // User {
    //     active: true,
    //     username: username,
    //     email: email,
    //     sign_in_count: 1,
    // }
}
