/// Notes from 2022 June 6
/// 
/// https://leetcode.com/problems/longest-substring-without-repeating-characters/discuss/379617/Rust-with-HashMap%3A%3AEntry
/// 
/// Psuedocode:
/// for every char in string
///     store new char in hashmap
///         if char is already in HM, reset counter and save new length if > old length
///         if char is not, update counter and length
/// 
/// 
/// 1. ::new() instead of .new()
/// Becuase there is no object new() is acting on, we call it from the library directly
/// with the :: syntax
/// 
/// 
/// 2. .insert(k: K, v: V) returns an Option<V>
/// If the key is present, returns an option with the old value wrapped in an option
/// If the key is not present, returns None
/// 
/// 3. if let statement https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
/// If let statements allow for the "if" evaluation of simple Options in the case where
/// a match would be cumbersome
/// In this case, the logic is "if there is some value"
/// No need to handle the None case in this example
/// 

use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut m = HashMap::new();
        let mut ans = 0;
        let mut before = -1;
        let mut current = 0;
        for c in s.chars() {
            if let Some(last) = m.insert(c, current) {
                before = max(before, last);
            }
            ans = max(ans, current - before);
            current += 1;
        }
        ans
    }
}