use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(&0);
        
        while let Some(current) = queue.pop_front() {
            if visited.contains(&current) {
                continue;
            } 
            for another in rooms[*current as usize].iter() {
                if visited.contains(&another) {
                    continue;
                }
                queue.push_back(another);
            }
            visited.insert(current);
            if visited.len() == rooms.len() {
                return true;
            }
        } 
        
        return (visited.len() == rooms.len());
    }
}
