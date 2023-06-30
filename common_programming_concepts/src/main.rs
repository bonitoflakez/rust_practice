fn main() {
    println!("immutable values");
    {
        let x = 5;
        println!("The value of x is {}\n", x);
    }

    println!("using mutable");
    {
        let mut x = 5;
        println!("unchanged {}", x);
        x = 10;
        println!("changed {}\n", x);
    }

    const SCORE: u32 = 100_00;
    {
        println!("{}\n", SCORE);
    }

    println!("shadowing");
    {
        let x = 5;
        let x = x + 1;
        {
            let x = x * 2;
            println!("inside scope {}", x);
        }

        println!("outside scope {}\n", x);
    }

    println!("size of string");
    {
        let spaces_str = "      ";
        let spaces_num = spaces_str.len();
        println!("size is {}\n", spaces_num);
    }

    println!("data types");
    {
        println!("floats");
        let float_1 = 2.0;
        let float_2: f32 = 3.0;

        println!("f64 float {}", float_1);
        println!("f32 float {}\n", float_2);
    }

    {
        println!("numeric operations");
        let sum = 5 + 10;
        let difference = 95.5 - 4.3;
        let product = 4 * 30;
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3;
        let remainder = 43 % 5;

        println!("sum value {}", sum);
        println!("difference value {}", difference);
        println!("product value {}", product);
        println!("quotient value {}", quotient);
        println!("truncated value {}", truncated);
        println!("remainder value {}\n", remainder);
    }

    {
        println!("boolean type");
        let t = true;
        let f: bool = false;

        println!("truthy {}", t);
        println!("falsy {}\n", f);
    }

    {
        println!("character type");
        let c = 'z';
        let z: char = 'Z';
        let heart_eyed_cat = '😻';

        println!("normal character {}", c);
        println!("character with explicit type annotation {}", z);
        println!("heart eyed cat {}\n", heart_eyed_cat);
    }

    println!("tuples");
    {
        let tup1: (i32, f64, u8) = (500, 6.4, 1);
        println!("values of 'tup1' {} {} {}", tup1.0, tup1.1, tup1.2);

        let tup2 = (500, 6.4, 1);
        let (x, y, z) = tup2;
        println!("values of 'tup2' {} {} {}\n", x, y, z);
    }

    println!("array");
    {
        let a = [1, 2, 3, 4, 5];
        let first = a[0];
        let second = a[1];
        println!("first and second element of array {} {}\n", first, second);
    }

    println!("functions");
    {
        print_val();
        take_num(3);
        println!("{}\n", fn_with_return());
    }

    println!("control flow");
    {
        // if else
        let number = 5;
        if 5 > number {
            println!("true");
        } else {
            println!("false");
        }

        // else if
        let new_number = 6;
        if new_number % 4 == 0 {
            println!("{} divisible by 4", new_number);
        } else if new_number % 3 == 0 {
            println!("{} divisible by 3", new_number);
        } else if new_number % 2 == 0 {
            println!("{} divisible by 2", new_number);
        } else {
            println!("{} is not divisible by 4, 3 or 2", new_number);
        }

        // if in let statement
        let condition = true;
        let number = if condition { 1 } else { 0 };

        println!("{}\n", number);
    }

    {
        println!("loops");
        let mut counter = 0;

        let res = loop {
            counter += 1;
            if counter == 5 {
                break counter * 2;
            }
        };

        println!("{}\n", res);
    }

    // loop labels to Disambiguate between multiple loops
    {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("final count = {count}\n");
    }

    // conditional loops with while
    {
        let mut number = 3;

        while number != 0 {
            println!("{number}");
            number -= 1;
        }

        println!("LIFTOFF!!!\n");
    }

    // looping through a collection with for
    {
        let a = [1, 2, 3, 4, 5];

        for elem in a {
            println!("{elem}");
        }

        println!("\n");

        for num in (1..4).rev() {
            println!("{num}");
        }
    }
}

fn print_val() {
    println!("yellow");
}

fn take_num(x: i32) {
    println!("got {}", x);
}

fn fn_with_return() -> i32 {
    let x = 10;
    return x;
}
