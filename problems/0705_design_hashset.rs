struct MyHashSet {
    table: Vec<bool>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyHashSet { table: vec![false; 1_000_000]}
    }
    
    fn add(&mut self, key: i32) {
        self.table[key as usize] = true;
    }
    
    fn remove(&mut self, key: i32) {
        self.table[key as usize] = false;
    }
    
    /** Returns true if this set contains the specified element */
    fn contains(&self, key: i32) -> bool {
        self.table[key as usize]
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */
