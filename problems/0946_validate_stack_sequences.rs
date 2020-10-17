impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = Vec::new();
        let mut i_pushed = 0;
        for i_popped in 0..popped.len() {
            if stack.is_empty() && !Self::push(&mut stack, &pushed, &mut i_pushed) {
                return false;
            }
            while let Some(last) = stack.last() {
                if *last == popped[i_popped] {
                    break;
                }
                if !Self::push(&mut stack, &pushed, &mut i_pushed) {
                    return false;
                }
            }
            stack.pop();
        }
        return true;
    }
    
    fn push(stack: &mut Vec<i32>, pushed: &Vec<i32>, i: &mut usize) -> bool {
        if *i == pushed.len() {
            return false;
        }
        stack.push(pushed[*i]);
        *i += 1;  
        return true;
    }
}
