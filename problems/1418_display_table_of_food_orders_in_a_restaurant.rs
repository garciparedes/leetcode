use std::collections::{
    HashMap,
    HashSet,
};

impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut queries = HashMap::new();
        let mut names = HashSet::new();
        for order in orders {
            let table = order[1].clone().parse::<usize>().unwrap();
            let name = order[2].clone();
            let v = queries
                .entry(table)
                .or_insert_with(HashMap::new)
                .entry(name.clone())
                .or_insert(0);
            *v += 1;
            names.insert(name);
        }      
        let mut names: Vec<String> = names.into_iter().collect();
        names.sort_unstable();
        
        let mut queries: Vec<_> = queries.into_iter().collect();
        queries.sort_unstable_by_key(|kv| kv.0);
        
        let mut rows: Vec<_> = queries
            .into_iter()
            .map(|(k, v)| {
                let mut row = Vec::new();
                row.push(k.to_string());
                for name in names.iter() {
                    row.push(v.get(name).unwrap_or(&0).to_string());
                }
                return row;
            })
            .collect();
        names.insert(0, String::from("Table"));
        rows.insert(0, names);
        return rows;
        
    }
}
