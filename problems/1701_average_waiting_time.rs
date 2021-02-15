impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let n = customers.len();
        let mut total = 0;
        let mut current = customers[0][0] as i64;
        for customer in customers {
            if current < customer[0] as i64 {
                current += customer[0] as i64 - current;
            }
            current += customer[1] as i64;
            total += current - customer[0] as i64;
        }
        
        return (total as f64) / (n as f64);
    }
}
