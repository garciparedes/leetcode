use std::cmp;

struct CustomStack {
    max_size: usize,
    container: Vec<i32>,
}

impl CustomStack {

    fn new(maxSize: i32) -> Self {
        CustomStack { 
            max_size: maxSize as usize, 
            container: Vec::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        if self.container.len() >= self.max_size {
            return;
        }
        self.container.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        match self.container.pop() {
            Some(x) => x,
            None => -1,
        }
    }
    
    fn increment(&mut self, k: i32, val: i32) {
        for i in 0..cmp::min(k as usize, self.container.len()) {
            self.container[i] += val;
        }
    }
}

