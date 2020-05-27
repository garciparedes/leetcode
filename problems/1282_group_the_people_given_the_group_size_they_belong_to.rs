use std::collections::HashMap;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut grouped: HashMap<i32, Vec<i32>> = HashMap::new();
        for (i, &g) in group_sizes.iter().enumerate() {
            grouped.entry(g).or_insert(Vec::new()).push(i as i32);
        }
        return grouped
            .iter()
            .map(|(&k, v)| {
                v.chunks(k as usize).map(|c| {
                    let mut tmp: Vec<i32> = Vec::new();
                    tmp.extend(c);
                    return tmp;
                }).collect::<Vec<Vec<i32>>>()
            })
            .flatten()
            .collect::<Vec<Vec<i32>>>()
        ;
    }
}
