pub fn public_function() {
    println!("called nlib's `public_function()`");
}

fn private_function() {
    println!("called nlib's `private_function()`");
}

pub fn indirect_access() {
    print!("called nlib's `indirect_access()`, that\n> ");

    private_function();
}