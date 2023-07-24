/// An iterative solution that creates windows of decreasing size, and slides them through the
/// input string. When it first finds a palindrome, it returns the substring.
///
/// Interestingly, I found that, when checking if a slice &[u8] is a palindrome, a manual for-loop
/// performs x2 worse than slice.iter() vs. slice.iter().rev(). I don't know why this is the case,
/// but I imagine rustc aggressively optimizes iterators to accomplish this.
///
/// Complexity:
///     Time: O(n^2)
///         Because we make n comparisons 1 time, then n-1 comparisons 2 times, and n-2 comparisons
///         3 times and so on and so forth, which is in the order of n^2.
///     Space: O(1)
///         Not counting the returned String, this algorithm requires no scaling space.
#[allow(dead_code)]
pub fn longest_palindrome(s: String) -> String {
    let mut window_size: usize = s.len();
    let chars: &[u8] = s.as_bytes();

    fn check_pali(slice: &[u8]) -> bool {
        if slice.len() < 2 { return true; }
        let iter = slice.iter();
        iter.clone().eq(iter.clone().rev())
    }

    while window_size > 0 {
        for slice in chars.windows(window_size) {
            if check_pali(slice) {
                return String::from_utf8(slice.to_vec()).unwrap();
            }
        }
        window_size -= 1;
    }
    panic!("Reached outside of while loop, should be impossible.")
}