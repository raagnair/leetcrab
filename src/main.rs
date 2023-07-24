use crate::solutions::s0005_longest_palindrome_substr::{longest_palindrome};
mod solutions;

fn main() {
    let rv = longest_palindrome("abcdcba".to_string());
    println!("Result {:?}", rv)
}