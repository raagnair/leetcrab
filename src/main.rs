use crate::solutions::s0008_string_to_integer::my_atoi;
mod solutions;

fn main() {
    let rv = my_atoi("2147483648".to_string());
    println!("Result {:?}", rv)
}