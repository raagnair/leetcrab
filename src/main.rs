use crate::solutions::add_two_numbers::_add_two_numbers;
mod solutions;

fn main() {
    let rv = _add_two_numbers(None, None);
    println!("Result {:?}", rv)
}