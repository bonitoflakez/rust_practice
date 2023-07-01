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
