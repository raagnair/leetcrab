/// The general approach here is to iterate forward through the String, maintaining a sliding
/// window that tries to capture the current repeat-less substring under evaluation.
///
/// Every time our right pointer encounters a letter that we already have in our substring,
/// we shift the window forward to exclude the previous occurrence of said letter. In order to
/// easily track which index the previous letters were encountered at, we populate a HashMap, then
/// remove from this HashMap every time our window slides forward.
///
/// At each letter, regardless of whether this adds to our window or slides it forward, we check
/// to see if our window length is greater than our longest encountered window length, and if yes,
/// we update our longest encountered length.
///
/// Return our longest encountered length.
#[allow(dead_code)]
pub fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::HashMap;
    let mut letter_to_idx: HashMap<u8, usize> = HashMap::new();
    let mut back_idx = 0;
    let mut longest_len = 0;

    let chars: &[u8] = s.as_bytes();

    for (idx, &byte) in chars.iter().enumerate() {
        match letter_to_idx.get(&byte) {
            Some(&old_idx) => {
                for i in back_idx..=old_idx { letter_to_idx.remove(&chars[i]); }
                letter_to_idx.insert(byte, idx);
                back_idx = old_idx + 1;
            }
            None => {
                letter_to_idx.insert(byte, idx);
            }
        }
        longest_len = longest_len.max(idx - back_idx + 1);
    }
    longest_len as i32
}