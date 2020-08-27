use crate::solution::Solution;

impl Solution {
    /// 给定一个字符串，请你找出其中不含有重复字符的`最长子串`的长度。
    ///
    /// ## 示例 1:
    /// ```
    /// 输入: "abcabcbb"
    /// 输出: 3
    /// 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3.
    /// ```
    ///
    /// ## 示例 2:
    /// ```
    /// 输入: "bbbbb"
    /// 输出: 1
    /// 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
    /// ```
    ///
    /// ## 示例 3:
    /// ```
    /// 输入: "pwwkew"
    /// 输出: 3
    /// 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
    ///      请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串.
    /// ```
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let (mut start, mut end, mut max) = (0, 0, 0);

        while end < chars.len() {
            for idx in start..end {
                if chars[idx] == chars[end] {
                    start = idx + 1;
                    break;
                }
            }
            let len = end - start + 1;
            if len > max {
                max = len
            }
            end += 1
        }
        max as i32
    }
}

#[test]
fn length_of_longest_substring_test() {
    assert_eq!(
        Solution::length_of_longest_substring(String::from("abcabcbb")),
        3
    );
    assert_eq!(
        Solution::length_of_longest_substring(String::from("bbbbb")),
        1
    );
    assert_eq!(
        Solution::length_of_longest_substring(String::from("pwwkew")),
        3
    );
}
