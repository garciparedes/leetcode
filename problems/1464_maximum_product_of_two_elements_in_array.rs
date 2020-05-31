impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut a, mut b) = (0, 0);
        for x in nums {
            if x > b {
                if x > a {
                    b = a;
                    a = x;
                } else {
                    b = x;
                }      
            } 
        }
        return (a - 1) * (b - 1);
    }
}
