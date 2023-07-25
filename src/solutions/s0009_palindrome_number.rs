/// Construct the opposite number by traversing left-ward from the smallest digit in the input, and
/// adding that to the opposite number after multiplying it by 10 at every step. At the end, check
/// if the input == opposite.
///
/// This is obviously less efficient than locating each pair of digits from outside to the center,
/// comparing, and short-circuiting upon finding a difference. But I felt that this solution is
/// far more readable.
///
/// Complexity:
///     Time: O(1)
///         Number of iterations scales with the number of digits in the input, which is log_10(n)
///         therefore in the order of log(n). Though with this specific problem-space, since it's
///         constrained to 32-bit integers, can be said to have a constant runtime.
///     Space: O(1)
///         At max we need to store 2 ints.
#[allow(dead_code)]
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {return false}
    if x < 10 {return true}

    let mut opposite = 0;
    let mut x_tracker = x.clone();
    while x_tracker > 0 {
        opposite = opposite * 10 + x_tracker % 10;
        x_tracker = x_tracker/10;
    }
    x == opposite
}