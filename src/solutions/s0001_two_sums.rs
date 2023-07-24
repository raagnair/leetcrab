use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut visited_nums: HashMap<i32, i32> = HashMap::new();
    for(index, elem) in nums.iter().enumerate() {
        let remainder = target - *elem;
        if visited_nums.contains_key(&remainder) {
            return vec![*visited_nums.get(&remainder).unwrap(), index as i32];
        } 
        visited_nums.insert(*elem, index as i32);
    }
    Vec::new()
}