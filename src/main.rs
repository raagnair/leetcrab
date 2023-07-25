use crate::solutions::s0006_zigzag_conversion::convert;
mod solutions;

fn main() {
    let rv = convert("PAYPALISHIRING".to_string(), 3);
    println!("Result {:?}", rv)
}