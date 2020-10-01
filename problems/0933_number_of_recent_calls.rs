use std::collections::VecDeque;

struct RecentCounter {
    pings: VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        RecentCounter { pings: VecDeque::new() }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        while let Some(last) = self.pings.back() {
            if t - last <= 3000 {
                break;
            }
            self.pings.pop_back();
        }
        self.pings.push_front(t);
        return self.pings.len() as i32;
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
