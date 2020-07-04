impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let bounds = Self::build_bounds(label);
        return Self::build_path(&bounds, label);
    }
    
    fn build_bounds(label: i32) -> Vec<i32> {
        let mut depth = 0;  
        let mut bounds = Vec::new();
        let (mut left, mut right) = (1, 1);
        while left <= label {
            let item = if depth % 2 == 0 { left } else { right };
            bounds.push(item);
            depth += 1;
            left = right + 1;
            right += i32::pow(2, depth);
         }
        return bounds;
    }
    
    fn build_path(bounds: &Vec<i32>, mut label: i32) -> Vec<i32> {
       let iterable = bounds
            .as_slice()
            .windows(2)
            .enumerate()
            .rev();
        
        let mut result = Vec::new();
        for (i, w) in iterable {
            let (head, tail) = (w[0], w[1]);
            result.push(label);
            if i % 2 == 0 {
                label = head + (tail - label) / 2;
            } else {
                label = head - (label - tail) / 2;
            }
        }
        result.push(1);
        
        return result
            .into_iter()   
            .rev()
            .collect(); 
    }
}
