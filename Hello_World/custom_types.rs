#![allow(dead_code)]

use crate::List::*;

enum List{
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List{
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1+ tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self{
            Cons(head, ref tail) => {
                format!("{} -> {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}


#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32{
    let Point {x: point_a, y: point_b} = rect.top_left;
    let Point {x: point_c, y: point_d} = rect.bottom_right;
    let edge1 = (point_a - point_c).abs();
    let edge2 = (point_b - point_d).abs();
    edge1*edge2
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64},
}

// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

//static: A possibly mutable variable with 'static lifetime. The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable static variable is unsafe.


fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click {x, y} => {
            println!("clicked at x={}, y={}.", x,y);
        },
    }
}

enum Status {
    Rich,
    Poor,
}

// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age};

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4};

    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point};

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let _rectangle = Rectangle{
        top_left: Point { x: 2.0, y: 2.0},
        bottom_right: Point { x: 5.0, y: 5.0},
    };

    println!(" rect points: {:?}, {:?}", _rectangle.top_left, _rectangle.bottom_right);

    let area = rect_area(_rectangle);

    println!(" rect area: {}", area);

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);


    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    use crate::Status::{Poor, Rich};

    let status = Poor;

    match status {
        Rich => println!("The rich have lots of money"),
        Poor => println!("the poor have no money.."),
    }

    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

}