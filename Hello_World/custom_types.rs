#![allow(dead_code)]

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

}