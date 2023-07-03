#![allow(unused)]

fn main() {
    {
        let mut v: Vec<i32> = Vec::new();
        // let v = vec![1, 2, 3];

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }

    {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("third element is {third}");

        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("third element is {third}"),
            None => println!("no third element"),
        }
    }

    // {
    // PANICS!!
    //     let v = vec![1, 2, 3, 4, 5];

    //     let does_not_exist = &v[100];
    //     let does_not_exist = v.get(100);
    // }

    // iteration
    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{i}");
        }
    }

    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
    }

    // storing multiple types in enum
    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }

    {
        let mut s = String::new();
    }

    {
        let data = "initial contents";

        let s = data.to_string();

        let s = "initial contents".to_string();
    }

    // updating a string
    {
        let mut s = String::from("foo");
        s.push_str("bar");
    }

    {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {s2}");
    }

    {
        let mut s = String::from("lo");
        s.push('l');
    }

    {
        let s1 = String::from("hello");
        let s2 = String::from("world");
        let s3 = s1 + &s2;
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{s1}-{s2}-{s3}");
        println!("{s}");
    }

    {
        let str = "yellow";

        let mut counter = 0;

        for c in str.chars() {
            counter += 1;
            print!("{c}");
        }

        println!("\nstr has {counter} characters")
    }
}
