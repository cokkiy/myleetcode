/*
 * @lc app=leetcode.cn id=2155 lang=rust
 *
 * [2155] 分组得分最高的所有下标
 */

// @lc code=start
impl Solution {
    pub fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
        let l = nums.len();
        let mut scores = Vec::with_capacity(l);
        let mut left_zero_count = 0;
        let mut right_one_count = 0;
        for i in 0..l {
            if nums[i] == 1 {
                right_one_count = right_one_count + 1;
            }
        }

        let mut highest_score = right_one_count;
        scores.push(highest_score);

        for i in 0..l {
            if nums[i] == 0 {
                left_zero_count = left_zero_count + 1;
            } else {
                right_one_count = right_one_count - 1;
            }
            let score = left_zero_count + right_one_count;
            scores.push(score);
            if score > highest_score {
                highest_score = score;
            }
        }

        let mut highest_scores: Vec<i32> = vec![];
        for i in 0..scores.len() {
            if scores[i] == highest_score {
                highest_scores.push(i as i32);
            }
        }

        return highest_scores;
    }
}
// @lc code=end
