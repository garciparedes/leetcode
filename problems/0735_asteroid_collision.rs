use std::cmp::Ordering;

impl Solution {
    pub fn asteroid_collision(mut asteroids: Vec<i32>) -> Vec<i32> {
        let mut changes = true;
        while changes && !asteroids.is_empty() {
            changes = false;
        
            let mut i = asteroids.len() - 1;
            while i > 0 {
                if (asteroids[i - 1] > 0) <= (asteroids[i] > 0) {
                    i -= 1;
                    continue;
                }
                changes = true;
                match asteroids[i - 1].abs().cmp(&asteroids[i].abs()) {
                    Ordering::Less => {
                        asteroids.remove(i - 1);
                        i -= 1;
                    },
                    Ordering::Equal => {
                        asteroids.remove(i);
                        asteroids.remove(i - 1);
                        i -= 1;
                        if i > 0 {
                            i -= 1;
                        }
                    },
                    Ordering::Greater => {
                        asteroids.remove(i);
                        i -= 1;
                    }
                }
            }
        }
        return asteroids;
    }
}
