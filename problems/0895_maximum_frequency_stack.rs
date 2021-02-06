use std::collections::HashMap;
use std::cmp;

struct FreqStack {
    counter: HashMap<i32, i32>,
    mapper: HashMap<i32, Vec<i32>>,
    max_freq: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {

    fn new() -> Self {
        Self { counter: HashMap::new(), mapper: HashMap::new(), max_freq: 0 }        
    }
    
    fn push(&mut self, x: i32) { 
        let freq = self.counter.entry(x).or_insert(0);
        *freq += 1;
        if *freq > self.max_freq {
            self.max_freq = *freq;
        }
        self.mapper.entry(*freq).or_insert_with(Vec::new).push(x);
    }
    
    fn pop(&mut self) -> i32 {
        let freqs = self.mapper.get_mut(&self.max_freq).unwrap();
        let ans = freqs.pop().unwrap();
        
        *self.counter.get_mut(&ans).unwrap() -= 1;
        if freqs.is_empty() {
            self.max_freq -= 1;
        }
        
        return ans;
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 */
