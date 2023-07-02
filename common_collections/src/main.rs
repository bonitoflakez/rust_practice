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

    {
        // PANICS!!
        let v = vec![1, 2, 3, 4, 5];

        let does_not_exist = &v[100];
        let does_not_exist = v.get(100);
    }

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
}
