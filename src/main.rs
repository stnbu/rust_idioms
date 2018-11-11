// Quirky, idiomatic Rust stuff...

fn get_element_n_of_string() {
    let s = "mybog";
    let chars: Vec<char> = s.chars().collect();
    println!("{}", chars[0]);
}

fn main() {
    get_element_n_of_string();
}
