impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        Self::create(nums)
            .into_iter()
            .map(|v| Self::maximize(v, maximum_bit))
            .collect()
    }
    
    fn create(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut xored = vec![nums[0]];
        for i in (1..n) {
            xored.push(xored[xored.len() - 1] ^ nums[i]);
        }
        xored.reverse();
        return xored;
    }
    
    fn maximize(v: i32, maximum_bit: i32) -> i32 {
        let mut ans = 0;
        for b in (0..maximum_bit) {
            if (v & (1 << b)) == 0 {
                ans |= (1 << b);
            }
        }
        return ans;
    } 
}
