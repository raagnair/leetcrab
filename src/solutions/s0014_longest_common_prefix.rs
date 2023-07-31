/// Pretty straight forward. Loop through the input Vector keeping track of the prefix at every
/// step, and comparing the latest item in the Vector to the latest prefix.
///
/// Complexity:
///     Time: O(n)
///         Need to iterate through the input once
///     Space: O(1)
///         No extra storage used.
#[allow(dead_code)]
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    fn find_common_prefix<'a>(left: &'a str, right: &'a str) -> &'a str {
        let mut idx: usize = 0;
        let left_chars: Vec<char> = left.chars().collect();
        let right_chars: Vec<char> = right.chars().collect();
        while idx < left.len().min(right.len()) {
            if left_chars.get(idx).unwrap() != right_chars.get(idx).unwrap() { break }
            idx += 1;
        }
        &left[0..idx]
    }
    let mut rv: &str = strs.get(0).unwrap().as_str();
    for idx in 1..strs.len() {
        rv = find_common_prefix(rv, strs.get(idx).unwrap().as_str());
    }

    rv.to_owned()
}