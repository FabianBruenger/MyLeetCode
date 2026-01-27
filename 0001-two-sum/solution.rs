impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // O(n2) solution. Longer time, less space. Brute Force.
    // for i in 0..nums.len() {
    //     for j in i+1..nums.len() {
    //         if nums[i] + nums[j] == target {
    //             return vec![i as i32, j as i32];
    //         }
    //     }
    // }

    // O(n) solution. Shorter time, more space.
    use std::collections::HashMap;
    let mut buffer_map: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        let key = nums[i]; // current value
        let value = target - key; // target value pair

        // Check if there is the key of value, because that means it was an array entry
        if let Some(value_index) = buffer_map.get(&key) {
            return vec![*value_index, i as i32]
        }

        // Insert an expected value for position. E.g expecting a 7 for the 2 at position 0. 
        buffer_map.insert(value, i as i32);
    }

    vec![]
    }
}
