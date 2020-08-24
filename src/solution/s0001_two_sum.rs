/*
 * 给定一个整数数组 nums 和一个目标值 target，
 * 请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
 *
 * 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。
 *
 * 示例:
 *
 * 给定 nums = [2, 7, 11, 15], target = 9
 * 因为 nums[0] + nums[1] = 2 + 7 = 9
 * 所以返回 [0, 1]
*/

use crate::solution::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::BTreeMap;
        let mut bmap: BTreeMap<i32, usize> = BTreeMap::new();
        let mut result: Vec<i32> = Vec::with_capacity(2);
        for (i, &v) in nums.iter().enumerate() {
            let key = &(target - v);
            if bmap.contains_key(key) {
                let j = *bmap.get(key).unwrap();
                result.insert(0, j as i32);
                result.insert(1, i as i32);
                break;
            }
            bmap.insert(v, i);
        }
        result
    }
}

#[test]
fn two_sum_test() {
    assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![1, 2], Solution::two_sum(vec![24, -10, 12, 34], 2));
    assert_eq!(vec![2, 3], Solution::two_sum(vec![2, 3, 9, -1], 8));
}
