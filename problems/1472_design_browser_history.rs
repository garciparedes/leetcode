struct BrowserHistory {
    history: Vec<String>,
    index: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {

    fn new(homepage: String) -> Self {
        BrowserHistory { history: vec![homepage], index: 0, }
    }
    
    fn visit(&mut self, url: String) {
        self.history.split_off(self.index + 1);
        self.history.push(url);
        self.index += 1;
    }
    
    fn back(&mut self, steps: i32) -> String {
        if self.index > 0 {
            let steps = steps as usize;
            if self.index > steps {
                self.index -= steps;
            } else {
                self.index = 0;
            }    
        }
        return self.history[self.index].clone();
    }
    
    fn forward(&mut self, steps: i32) -> String {
        if self.index < self.history.len() - 1 {
            let steps = steps as usize;
            if self.index + steps < self.history.len() {
                self.index += steps;
            } else {
                self.index = self.history.len() - 1;
            }    
        }
        return self.history[self.index].clone();
    }
}

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * let obj = BrowserHistory::new(homepage);
 * obj.visit(url);
 * let ret_2: String = obj.back(steps);
 * let ret_3: String = obj.forward(steps);
 */
