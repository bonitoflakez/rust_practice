#![allow(unused)]

use generic_types_traits_nd_lifetimes::{NewsArticle, Summary, Tweet};
use std::fmt::Display;

fn main() {
    {
        let number_list = vec![34, 50, 25, 100, 65];
        let largest = find_largest_i32(&number_list);
        println!("Largest number in list: {}", largest);
    }

    {
        let number_list = vec![102, 34, 6000, 90, 54, 2, 43, 8];
        let largest = find_largest_i32(&number_list);
        println!("Largest number in list: {}", largest);
    }

    {
        let number_list = vec![34, 50, 25, 100, 65];
        let largest = find_largest(&number_list);
        println!("Largest number in list: {}", largest);

        let char_list = vec!['y', 'm', 'a', 'q'];
        let largest = find_largest(&char_list);
        println!("Largest character in list: {}", largest);
    }

    {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        let both_integer = Point { x: 5, y: 10 };
        let both_float = Point { x: 1.0, y: 4.0 };
        let integer_and_float = Point { x: 5, y: 4.0 };
    }

    {
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            fn new(x: T, y: T) -> Self {
                Point { x, y }
            }

            fn x(&self) -> &T {
                &self.x
            }
        }

        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        let p = Point::new(5, 10);
        println!("p.x = {}", p.x());
    }

    {
        struct Point<X1, Y1> {
            x: X1,
            y: Y1,
        }

        impl<X1, Y1> Point<X1, Y1> {
            fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };
        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}\n", p3.x, p3.y);
    }

    {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
    }

    {
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
        };

        println!("New article available!\n{}\n", article.summarize());
    }

    {
        let str1 = String::from("abcd");
        let str2 = String::from("xyz");

        let result = longest(str1.as_str(), str2.as_str());
        println!("{result}\n");
    }

    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };

        println!("{}\n", first_word(&novel));
    }

    {
        let str1 = String::from("abcd");
        let str2 = "xyz";

        let result =
            longest_with_an_announcement(str1.as_str(), str2, "Today is someone's birthday!");
        println!("The longest string is {}", result);
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn find_largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn find_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
