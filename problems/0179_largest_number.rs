impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums: Vec<_> = nums
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        
        nums.sort_unstable_by(|a, b| format!("{}{}", b, a).cmp(&format!("{}{}", a, b)));
        
        let mut best = nums.join("");
        if &best[..1] == "0" {
            while best.len() > 1 && &best[best.len() - 1..] == "0" {
                best.pop();
            }   
        }
        return best;
    }
}
