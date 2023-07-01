#![allow(unused)]
fn main() {
    {
        let s = "hello";
    }

    {
        let mut s = String::from("hello");

        s.push_str(", world");

        println!("{}\n", s);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("{}, world\n", s2);
    }

    {
        let x = 5;
        let y = x;

        println!("{} {}", x, y);
    }

    // ownership and functions
    {
        let s = String::from("hello");

        takes_ownership(s);

        let x = 5;

        makes_copy(x);
    }

    // return values and scope
    {
        let s1 = gives_ownership();

        let s2 = String::from("hello");

        let s3 = takes_and_gives_back(s2);
    }

    {
        let s1 = String::from("hello");

        let (s2, len) = calculated_length(s1);

        println!("length of {} is {}\n", s2, len);
    }

    println!("references and borrowing");
    {
        let s1 = String::from("hello");

        let len = calculate_len_2(&s1);

        println!("len of {} is {}\n", s1, len);
    }

    {
        let mut s = String::from("hello");
        change(&mut s);
    }

    {
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
            println!("{r1}");
        }
        let r2 = &mut s;
        println!("{r2}\n");
    }

    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2);

        let r3 = &mut s;
        println!("{r3}\n");
    }

    {
        let reference_to_nothing = dangle();
    }

    // the slice type
    {
        let mut s = String::from("hello world");

        let word = first_word(&s);

        s.clear();
    }

    {
        let mut s = String::from("hello world");

        let word = first_word(&s);

        println!("{word}");
    }

    {
        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3];

        assert_eq!(slice, &[2, 3]);
    }
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

fn dangle() -> String {
    let s = String::from("hello");

    return s;
}

fn change(str: &mut String) {
    str.push_str(", world");
}

fn calculate_len_2(s: &String) -> usize {
    s.len()
}

fn calculated_length(s: String) -> (String, usize) {
    let length = s.len();

    return (s, length);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    return a_string;
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
