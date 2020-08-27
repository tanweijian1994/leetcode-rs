use crate::solution::Solution;
use crate::structures::linked_list::ListNode;

impl Solution {
    /// 给出两个`非空`的链表用来表示两个`非负`的整数。
    /// 其中，它们各自的位数是按照`逆序`的方式存储的，并且它们的每个节点只能存储`一位`数字。
    /// 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
    /// 
    /// 您可以假设除了数字 0 之外，这两个数都不会以 0 开头。
    /// 
    /// ## 示例：
    /// ```
    /// 输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
    /// 输出：7 -> 0 -> 8
    /// 原因：342 + 465 = 807
    /// ```
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy_head;
        let (mut l1_end, mut l2_end, mut overflow) = (false, false, false);
        loop {
            let lhs = match l1 {
                Some(node) => {
                    l1 = node.next;
                    node.val
                }
                None => {
                    l1_end = true;
                    0
                }
            };
            let rhs = match l2 {
                Some(node) => {
                    l2 = node.next;
                    node.val
                }
                None => {
                    l2_end = true;
                    0
                }
            };
            // if l1, l2 end and there is not overflow from previous operation, return the result
            if l1_end && l2_end && !overflow {
                break dummy_head.unwrap().next;
            }
            let sum = lhs + rhs + if overflow { 1 } else { 0 };
            let sum = if sum >= 10 {
                overflow = true;
                sum - 10
            } else {
                overflow = false;
                sum
            };
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            tail = &mut tail.as_mut().unwrap().next
        }
    }
}

#[test]
fn add_two_numbers_test() {
    assert_eq!(
        Solution::add_two_numbers(ListNode::vec_to_linked_list(vec![2, 4, 3]), ListNode::vec_to_linked_list(vec![5, 6, 4])),
        ListNode::vec_to_linked_list(vec![7, 0, 8])
    );
    assert_eq!(
        Solution::add_two_numbers(ListNode::vec_to_linked_list(vec![1, 2, 3]), ListNode::vec_to_linked_list(vec![8, 9, 1])),
        ListNode::vec_to_linked_list(vec![9, 1, 5])
    );
    assert_eq!(
        Solution::add_two_numbers(ListNode::vec_to_linked_list(vec![6, 4, 8]), ListNode::vec_to_linked_list(vec![5, 6, 9])),
        ListNode::vec_to_linked_list(vec![1, 1, 8, 1])
    );
}
