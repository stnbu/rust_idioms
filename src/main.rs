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


struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}
use std::thread;
use std::time::Duration;
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
//////////

fn main() {
    get_element_n_of_string();
    misreuse_of_closure();
    bytes_to_utf8();
    generate_workout(4, 7);
}
