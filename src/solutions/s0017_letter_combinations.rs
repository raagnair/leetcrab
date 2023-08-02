/// The problem seems to beg for a recursive approach, but my approach flattens it to an iterative
/// one instead.
///
/// The approach requires storing all the current strings produced by the previous number. So at the
/// start of the algorithm we'd initialize this as an empty Vector.
///
/// We traverse through each number provided in the input, and grab the list of chars
/// associated with that number on a keypad. Then for each string in our incrementing storage of
/// Strings, we add each letter and store the separate results as separate Strings.
///
/// This means if we have 6 Strings stored so far, and our current number maps to 4 chars,
/// we will now have 24 Strings.
///
/// In the end we return the latest Vector of Strings.
///
/// Complexity:
///     Time: O(4^n)
///         Assuming the worst case where the input is full of 8s or 9s, we have 4 possibilities
///         at each step, for n different steps, leading to 4^n possible outputs we have to touch.
///     Space: O(4^n)
///         Same reason as above, since the algorithm requires us to store intermediary steps
///         beyond just what the output is.
#[allow(dead_code)]
pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.len() == 0 { return vec![] }

    let registry: Vec<Vec<char>> = vec![
        vec![], vec![], vec!['a', 'b', 'c'], vec!['d', 'e', 'f'],
        vec!['g', 'h', 'i'], vec!['j', 'k', 'l'], vec!['m', 'n', 'o'],
        vec!['p', 'q', 'r', 's'], vec!['t', 'u', 'v'], vec!['w', 'x', 'y', 'z']
    ];

    let mut last_iter = vec![];
    let mut curr_iter: Vec<String> = vec!["".to_string()];

    for i in digits.chars() {
        last_iter = curr_iter.clone();
        curr_iter = vec![];

        let num = i.to_digit(10).unwrap() as usize;
        let curr_chars = registry.get(num).unwrap();
        for base_str in last_iter {
            for &curr_char in curr_chars {
                let mut to_push = base_str.clone();
                to_push.push(curr_char);
                curr_iter.push(to_push);
            }
        }
    }

    curr_iter
}