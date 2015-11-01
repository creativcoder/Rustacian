use std::io;

// defining a macro for reading inputs to buffer
macro_rules! scanline {
    ($x:expr) => {
                    io::stdin().read_line(&mut $x).ok().expect("I/O Error");
    };
}

fn main() {
    let mut input = String::new();
    scanline!(input);
    println!("{:?}",input);
}
