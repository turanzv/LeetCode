/// Notes 2022 June 6
/// 
/// Solution from https://leetcode.com/problems/add-two-numbers/discuss/469977/Simple-Rust-solution-less0ms-2.1MBgreater
/// 
/// Some(n) is the way to return the value in a Option enum
/// 
/// The Some(n) syntax is necessary because we are wrapping
/// an item whose type and properties are not known - after
/// all, the Option enum is sumply a wrapper for a generic
/// type and None
/// 
/// The value of the generic type in scope will be the name
/// given in the parentheses
/// 

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        match(l1, l2) {
            (None, None) => None,
            (Some(n), None) => {
                Some(n)
            },
            // Next two patterns can be shortened to this:
            // (Some(n), None) || (None, Some(n)) => Some(n),
            (None, Some(n)) => {
                Some(n)
            },
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val;
                
                if sum < 10 {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Solution::add_two_numbers(n1.next, n2.next)
                    }))
                } else {
                    let carry = Some(Box::new(ListNode::new(1)));
                    Some(Box::new(ListNode {
                        val: sum - 10,
                        next: Solution::add_two_numbers(
                            Solution::add_two_numbers(carry, n1.next), n2.next)
                    }))
                }
            },
        }
    }
}