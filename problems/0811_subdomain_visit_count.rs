use std::collections::HashMap;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut counter = HashMap::new();
        
        for domain in cpdomains.into_iter() {
            let raw: Vec<_> = domain.trim().split(" ").collect();
            let count = raw[0].parse::<i32>().unwrap(); 
            let mut current = raw[1].to_string();
            loop {
                *counter.entry(current.clone()).or_insert(0) += count;
                let mut iterator = current.splitn(2, '.');
                iterator.next().unwrap();
                current = match iterator.next() {
                    Some(item) => item.to_string(),
                    None => break,
                };
            }
        }
        return counter
            .into_iter()
            .map(|(item, count)| format!("{} {}", count, item))
            .collect();
    }
}
