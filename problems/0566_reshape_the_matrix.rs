impl Solution {
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let (r, c) = (r as usize, c as usize);
        let (n, m) = (nums.len(), nums[0].len());
        
        if r * c != n * m {
            return nums;
        }
        
        let mut ans = vec![vec![0; c]; r];
        for i in 0..n {
            for j in 0..m {
                let k = i * m + j;
                let (x, y) = (k / c, k % c);
                ans[x][y] = nums[i][j];
            }
        }
        return ans;
    }
}
