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

#[test]
fn two_sum_works() {
    assert_eq!(vec![0, 1], two_sum(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![1, 2], two_sum(vec![24, -10, 12, 34], 2));
    assert_eq!(vec![2, 3], two_sum(vec![2, 3, 9, -1], 8));
}
