use std::collections::BTreeMap;
use std::cmp;

struct MyCalendarThree {
    timeline: BTreeMap<i32, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {

    fn new() -> Self {
        Self { timeline: BTreeMap::new() }
    }
    
    fn book(&mut self, start: i32, end: i32) -> i32 {
        *self.timeline.entry(start).or_insert(0) += 1;
        *self.timeline.entry(end).or_insert(0) -= 1;
        
        let mut ans = 0;
        let mut current = 0;
        for diff in self.timeline.values() {
            current += diff;
            ans = cmp::max(ans, current);
        }
        
        return ans;
    }
}
    
/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(start, end);
 */
