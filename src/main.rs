use crate::solutions::s0013_roman_to_integer::roman_to_int;
mod solutions;

fn main() {
    let rv = roman_to_int("XCII".into());
    println!("Result {:?}", rv)
}