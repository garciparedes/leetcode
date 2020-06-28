impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let (mut minimum, mut maximum, mut cumulated) = (std::f64::MAX, std::f64::MIN, 0 as f64); 
        for &s in salary.iter() {
            let s = s as f64;
            if minimum > s {
                minimum = s;
            }
            if maximum < s {
                maximum = s;
            }
            cumulated += s;
        }
        
        return (cumulated - minimum - maximum) / (salary.len() - 2) as f64;
    }
}
