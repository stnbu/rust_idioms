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

fn main() {
    get_element_n_of_string();
    misreuse_of_closure();
}
