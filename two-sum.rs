use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut solution: Vec<i32> = Vec::new();
        for (i, num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                Some(v) => {
                    solution.push(*v);
                    solution.push(i as i32);
                    break;
                },
                None => {
                    map.insert(*num, i as i32);
                }
            }
        }
        return solution;
    }
}
