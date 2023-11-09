/*
 * @lc app=leetcode.cn id=50 lang=rust
 *
 * [50] Pow(x, n)
 */

// @lc code=start
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        if n == 1 {
            return x;
        }
        if n == -1 {
            return 1.0 / x;
        }
        let half = Solution::my_pow(x, n / 2);
        let rest = Solution::my_pow(x, n % 2);
        half * half * rest
    }
}
// @lc code=end

// Result:
/*
  Accepted
    306/306 cases passed (0 ms)
    Your runtime beats 100 % of rust submissions
    Your memory usage beats 56.45 % of rust submissions (2 MB)
*/
