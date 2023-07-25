use crate::solutions::s0009_palindrome_number::is_palindrome;
mod solutions;

fn main() {
    let rv = is_palindrome(1234567);
    println!("Result {:?}", rv)
}