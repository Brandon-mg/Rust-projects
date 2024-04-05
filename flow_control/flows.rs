#![allow(unreachable_code, unused_labels)]

use std::str::FromStr;


fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{}'", s);
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{}'", count_str);
    };
    (count, item)
}

enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well.
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon.
        };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);

    let mut count = 0u32;

    'looplabelouter: loop {
        println!("{}", count);
        count += 1;
        if count == 5 {
            'looplabelinner: loop{
                count+=1;
                println!("innerloop {}", count);
                break 'looplabelouter;
            }
            println!("unreachable");
            break;
        }
    }

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };
    println!("{}", result);
    assert_eq!(result, 20);

    while count < 100 {
        if count % 15 == 0{
            println!("fizzbuzz");
        } else if count % 3 == 0{
            println!("fizz");
        } else if count % 5 == 0{
            println!("buzz");
        } else {
            println!("{}", count)
        }
        count += 1;
    }

    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let names1 = vec!["Bob", "Frank", "Ferris"];

    for name in names1.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names1);

    let names2 = vec!["Bob", "Frank", "Ferris"];

    for name in names2.into_iter() {M 
        // explicitly handling the failing case.
    }
    // ^ `if let` had additional optional `else`/`else if`
    // clauses. `while let` does not have these.
}