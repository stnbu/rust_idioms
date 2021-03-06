// Quirky, idiomatic Rust stuff...

#[allow(dead_code)]
fn get_element_n_of_string() {
    let s = "mybog";
    let _chars: Vec<char> = s.chars().collect();
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

fn s_truct() {
    struct Bob(u8, u8);
    let _bob = Bob(1, 1);
    _bob.0;
    _bob.1;
    struct Mary {
        staff: u8,
        rod: u8,
    };
    let mut _m = Mary {
        staff: 1,
        rod: 1,
    };
    _m.staff;
    _m.rod;
    _m.rod = 2;
    let _n = Mary {
        staff: 1,
        rod: 1,
    };
    //_n.rod = 7;  // will fail to compile
    //let _o = Mary(1, 1);  // oddly does not work
}

fn e_num() {
    enum CumPass {
        North, East, West, South,
    }
    let _enum = CumPass::West;
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

fn wat() {
    type _Dingus = &'static str;
    // what the heck is going on here? I the book says...
    struct Zoo {
        n: _Dingus,
    }
    let _z = Zoo { n: "foo" };
    // ...In what other context can I use _Dingus?
}

fn m_atch() {
    let loki = ("Loki", true, 800u32);
    let mut _foo = match loki {
        (_, demi, _) if demi => 0,
        (name, _, _) if name == "Bob" => 1,
        (_, _, pow) if pow <= 1000 => 2,
        _ => 3,  // and the other things
    };
    _foo = 99;
}

fn o_ption() {
    let mut rng = 0..7;
    rng.next();
    rng.next();
    loop {
        match rng.next() {
            Some(n) => n,
            None => break,
        };
    }
}

fn r_everse_iter() {
    let x = [1, 2, 3];
    for y in x.iter().rev() {  // &x.rev() does not work!
        assert!(*y != 0);  // a star is born
    }
}

fn string_iter() {
    let s = "bob has fleas";
    for _ in s.split(" ") {}
    for _ in s.chars() {}
}

fn iter_consumer() {
    let x = 1..99999;
    // I believe this is ``list(xrange(1, 99999))`` ..no?
    let _y = x.collect::<Vec<i32>>();
    let x = 1..99999;
    let _y: Vec<i32> = x.collect();  // prettier
}

//fn f_ind() {}

fn f_ilter() {
    let rng = 1..1000;
    let rng_even = rng.filter(|n| *n % 2 == 0)
        .collect::<Vec<i32>>();
    for n in rng_even {
        assert!(n > 0);
    }
}

fn take_5() {
    let rng = 1..1000_i32;
    let rng_even_pow3 = rng.filter(|n| *n % 2 == 0)
        .map(|n| n.pow(3))  // why no ``*`` ?
        .take(5)  // yay
        .collect::<Vec<i32>>();
    #[allow(path_statements)]  // what is a path_statement?
    for _n in rng_even_pow3 {
        _n;
    }
}

fn f_old() {
    let sum = (0..101).fold(0, |sum, n| sum + n);
    assert_eq!(sum, 5050);
}

fn main() {
    f_old();
    take_5();
    f_ilter();
    iter_consumer();
    string_iter();
    r_everse_iter();
    o_ption();
    get_element_n_of_string();
    misreuse_of_closure();
    bytes_to_utf8();
    hmap();
    s_truct();
    e_num();
    m_atch();
}
