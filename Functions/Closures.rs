fn apply<F>(f: F) where
    F: FnOnce() {
        f();
}


fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
        f(3)
}


fn main() {
    use std::mem;

    let outer_var = 42;

    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred  = |i     |          i + outer_var  ;

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();

    let greeting = "hello";

    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greeting);

        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzz");

        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2*x;
    println!("3 doubled: {}", apply_to_3(double));

    let y = 7;

    let print = || println!("{}", y);

    apply(print);
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x ==2));
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x ==2));
    let vec3 = vec![1, 2, 3];
    let vec4 = vec![4, 5, 6];
    let mut iter = vec3.iter();
    let mut into_iter = vec4.into_iter();
    println!("Find 2 in vec3: {:?}", iter     .find(|&&x| x == 2));
    println!("Find 2 in vec4: {:?}", into_iter.find(| &x| x == 2));
    println!("vec1 len: {}", vec1.len());
    println!("First element of vec1 is: {}", vec1[0]);

    let vec = vec![1, 9, 3, 3, 13, 2];
    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));
    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);
}