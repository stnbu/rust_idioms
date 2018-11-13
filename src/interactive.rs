fn o_ption() {
    println!("what's your name, noble worrior?");
    let mut buf = String::new();
    use std::io;
    io::stdin().read_line(&mut buf)
        .ok()
        .expect("Failed to read stdin");
    let buf = buf.trim();
    println!("{}, that's a mighty name indeeeeed.", buf);
}

fn p_arse() {
    let mut buf = String::new();
    println!("Enter a number: ");
    use std::io;
    io::stdin().read_line(&mut buf)
        .ok()
        .expect("Not a number!");
    let input_num: Result<u32, _> = buf.trim().parse();
    match input_num {
        Ok(num) => println!("You entered the number {}", num),
        Err(ex) => println!("Cannot be parsed as a number: {}", ex)
    };
}

fn main() {
    o_ption();
    p_arse();
}
