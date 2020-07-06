impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut depth = Vec::with_capacity(seq.len());
        let mut max_depth = 0;
        for item in seq.chars() {
            if item == '(' {
                stack.push(1);
                depth.push(stack.len());
            } else {
                depth.push(stack.len());
                stack.pop();
            }
            if max_depth < stack.len() {
                max_depth = stack.len();
            }
        }
        
        return depth
            .into_iter()
            .map(|x| (x <= max_depth / 2) as i32)
            .collect();
    }
}
