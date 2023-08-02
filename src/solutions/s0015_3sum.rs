use std::collections::HashMap;
use std::collections::HashSet;

/// This approach sorts the array first, then uses three pointers to try to find valid three-sum
/// combos that add up to 0. The left-most pointer moves from left to right, and at each point it
/// creates a mid-pointer and right-pointer, located at the left + 1 index and at the length - 1
/// index respectively. These latter two pointers converge to the center of the remaining Vector
/// on the right side of the left-most pointer.
///
/// Complexity:
///     Time: O(n^2)
///         We are touching about every other element of the array, for every element we iterate.
///     Space: O(1)
///         No extra storage beyond the output.
#[allow(dead_code)]
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut rv_set: HashSet<Vec<i32>> = HashSet::new();

    let mut last_left = *nums.get(0).unwrap() - 1;

    for idx_left in 0..nums.len() {
        let left = *nums.get(idx_left).unwrap();
        if left > 0 { break; }
        if left == last_left { continue; }
        last_left = left;

        let mut idx_mid = idx_left + 1;
        let mut idx_right = nums.len() - 1;

        while idx_mid < idx_right {
            let mid = *nums.get(idx_mid).unwrap();
            let right = *nums.get(idx_right).unwrap();

            if left + mid > 0 { break; }

            let sum = left + mid + right;
            if sum == 0 {
                rv_set.insert(vec![left, mid, right]);
                idx_mid +=1;
                idx_right -= 1;
            }
            else if sum < 0 { idx_mid += 1; }
            else if sum > 0 { idx_right -= 1; }
        }
    }

    let mut rv: Vec<Vec<i32>> = vec![];
    for set in rv_set {
        rv.push(set);
    }
    rv
}

/// Here, we create a frequency map, then traverse through the elements of the original Vector input
/// to try to find pairs of numbers in the rest of the map that add up to -1 * current input number.
///
/// Complexity:
///     Time: O(n^2)
///         For every element, look at every other element.
///     Space: O(n)
///         Store all elements into the frequency map.
#[allow(dead_code)]
pub fn three_sum_works(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let map_of_freq: HashMap<i32, i32> = nums.iter().copied().fold(HashMap::new(), |mut map, v| {
        *map.entry(v).or_insert(0) += 1;
        map
    });

    fn process(num: i32, map: &HashMap<i32, i32>) -> HashSet<Vec<i32>> {
        let target = -1 * num;
        let mut rv: HashSet<Vec<i32>> = HashSet::new();

        for &first_val in map.keys() {
            let looking_for = target - first_val;
            let first_freq = *map.get(&first_val).unwrap();
            let second_freq = *map.get(&looking_for).unwrap_or(&0);

            if second_freq == 0 { continue }

            let is_valid: bool = if num == first_val && num == looking_for { first_freq >= 3 }
            else if num == first_val { first_freq >= 2 }
            else if num == looking_for { second_freq >= 2 }
            else if first_val == looking_for { first_freq >= 2 }
            else { true };

            if is_valid {
                let mut to_push = vec![num, first_val, looking_for];
                to_push.sort();
                rv.insert(to_push);
            }
        }

        rv
    }

    let mut rv_set: HashSet<Vec<i32>> = HashSet::new();

    for &key in map_of_freq.keys() {
        for v in process(key, &map_of_freq) {
            rv_set.insert(v);
        }
    }

    let mut rv: Vec<Vec<i32>> = vec![];
    for set in rv_set {
        rv.push(set);
    }
    rv
}

/// This approach stores all sums before-hand, in a HashMap that maps from sum -> a set of tuples
/// containing the numbers added together to make the sum. Then we loop through our input array
/// and look up elements in the HashMap keyed by -1 * the current element. Then we have some logic
/// to handle cases where we might be double-counting ourselves. Ultimately we add this to a set
/// and output the contents of the set.
///
/// Complexity:
///     Time: O(n^2)
///         Since two-sum is an O(n) problem, I believe three-sum must necessarily be an O(n^2)
///         problem since at each element we are trying to solve two-sum for the other elements.
///     Space: O(n^2)
///         This algorithm pre-computes all possible sums between n elements x n elements, thus
///         consuming O(n^2) space.
///
/// Failure:
///     I believe we can get away with not using so much space.
#[allow(dead_code)]
pub fn three_sum_slow(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let map_of_freq: HashMap<i32, i32> = nums.iter().copied().fold(HashMap::new(), |mut map, v| {
        *map.entry(v).or_insert(0) += 1;
        map
    });

    let mut map_of_sums: HashMap<i32, HashSet<(i32, i32)>> = HashMap::new();
    for idx_1 in 0..nums.len() {
        for idx_2 in idx_1+1..nums.len() {
            let num1 = *nums.get(idx_1).unwrap();
            let num2 = *nums.get(idx_2).unwrap();

            let sum = num1 + num2;
            let registry = (num1, num2);

            map_of_sums.entry(sum).or_insert(HashSet::new()).insert(registry);
        }
    }

    let mut rv_set: HashSet<Vec<i32>> = HashSet::new();

    for &key in map_of_freq.keys() {
        match map_of_sums.get(&(-1 * key)) {
            Some(&ref set) => {
                for (a, b) in set {
                    let a = *a;
                    let b = *b;
                    let valid_result: bool = if key == a && key == b {
                            *map_of_freq.get(&key).unwrap() >= 3
                        } else if key == a || key == b {
                            *map_of_freq.get(&key).unwrap() >= 2
                        } else { true };
                    if valid_result {
                        let mut to_push = vec![key, a, b];
                        to_push.sort();
                        rv_set.insert(to_push);
                    }
                }
            }
            None => {}
        }
    }

    let mut rv: Vec<Vec<i32>> = vec![];
    for set in rv_set {
        rv.push(set);
    }
    rv
}

