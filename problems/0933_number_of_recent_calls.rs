use std::collections::VecDeque;

struct RecentCounter {
    pings: VecDeque<i32>,
}

impl RecentCounter {

    fn new() -> Self {
        RecentCounter {
            pings: VecDeque::new(), 
        }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        self.pings.push_back(t);
        while let Some(&v) = self.pings.front() {
            if v >= t - 3000 {
                break;
            }
            self.pings.pop_front();
        }
        return self.pings.len() as i32;
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
