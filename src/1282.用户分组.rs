/*
 * @lc app=leetcode.cn id=1282 lang=rust
 *
 * [1282] 用户分组
 */
#![crate_name = "sln_1282"]
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut dict: HashMap<i32, Vec<i32>> = HashMap::new();
        for i in 0..group_sizes.len() {
            let key = group_sizes[i];
            let vec = dict.entry(key).or_insert_with(Vec::new);
            vec.push(i as i32);
        }
        let mut groups: Vec<Vec<i32>> = vec![];
        for (k, v) in dict {
            for chunk in v.chunks(k as usize) {
                groups.push(chunk.to_vec());
            }
        }
        return groups;
    }
}
// @lc code=end

// result:
// Accepted
// 103/103 cases passed (4 ms)
// Your runtime beats 71.43 % of rust submissions
// Your memory usage beats 42.86 % of rust submissions (2.2 MB)
