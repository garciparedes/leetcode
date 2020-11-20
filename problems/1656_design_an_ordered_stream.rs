struct OrderedStream {
    items: Vec<Option<String>>,
    index: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {

    fn new(n: i32) -> Self {
        Self { items: vec![None; n as usize], index: 0, }
    }
    
    fn insert(&mut self, id: i32, value: String) -> Vec<String> {
        self.items[(id - 1) as usize] = Some(value);
        
        let mut ans = Vec::new();
        while let Some(Some(item)) = self.items.get(self.index) {
            ans.push(item.clone());
            self.index += 1;
        }
        return ans;
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(id, value);
 */
