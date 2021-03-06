impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let n = boxes.len();
        let boxes: Vec<char> = boxes.chars().collect();
        let mut ans = vec![0; n];
        
        let (mut ones, mut cum) = (0, 0);
        for i in 0..n {
            ans[i] = cum;
            if boxes[i] == '1' {
                ones += 1
            }
            cum += ones; 
        }
        
        let (mut ones, mut cum) = (0, 0);
        for i in (0..n).rev() {
            ans[i] += cum;
            if boxes[i] == '1' {
                ones += 1
            }
            cum += ones; 
        }
        
        return ans;
    }
}
