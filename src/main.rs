// Quirky, idiomatic Rust stuff...

fn get_element_n_of_string() {
    let s = "mybog";
    let chars: Vec<char> = s.chars().collect();
    println!("{}", chars[0]);
}

fn misreuse_of_closure() {
    let example_closure = |x| x;
    let _s = example_closure(String::from("hello"));
    //let n = example_closure(5);  // cannot do this!
}

fn bytes_to_utf8() {
    use std::str;
    // some bytes, in a stack-allocated array
    let sparkle_heart = [240, 159, 146, 150];
    // We know these bytes are valid, so just use `unwrap()`.
    let sparkle_heart = str::from_utf8(&sparkle_heart).unwrap();
    assert_eq!("💖", sparkle_heart);
}

use std::collections::HashMap;
fn hmap() {
    let mut book_reviews = HashMap::new();
    book_reviews.insert(3, "foo");
    let key = 3;
    let x = match book_reviews.get(&key) {
        Some(value) => value,
        None => "",
    };
    assert_eq!(x, "foo");
}

fn main() {
    get_element_n_of_string();
    misreuse_of_closure();
    bytes_to_utf8();
    hmap();
}
