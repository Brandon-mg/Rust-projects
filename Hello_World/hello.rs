// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    println!("Hello World!");
    println!("I'm a Rustacean!");
    // string formatting
    println!("{} var1", 10);
    println!("{0}{1},FLIP,{1}{0}", "A", "B");
    println!("{subj} {verb} {obj}",
            obj="replacement",
            subj="label",
            verb="based");
    println!("Bases");
    println!("10:{}",12345);
    println!("2:{:b}",12345);
    println!("16:{:X}",12345);
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));
}