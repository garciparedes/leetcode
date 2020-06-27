impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut changes = 0;
        for item in s.chars() {
            if item == '(' {
                stack.push(item);
            } else {
                loop {
                    match stack.pop() {
                        Some(popped) => {
                            if popped == '(' {
                                break;
                            }
                            changes += 1;
                        }
                        None => {
                            changes += 1;
                            break;   
                        }
                    }
                }
            }
        }
        changes += stack.len() as i32;
        
        return changes;
    }
}
