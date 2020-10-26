use std::collections::HashSet;

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let s: Vec<_> = s.chars().map(|c| c as u8 - '0' as u8).collect();
        let mut memory = HashSet::new();
        Self::rec(s, a as u8, b as usize, &mut memory);
        
        return memory
            .into_iter()
            .min()
            .unwrap()
            .into_iter()
            .map(|v| (v + '0' as u8) as char)
            .collect();
    }
    
    fn rec(s: Vec<u8>, a: u8, b: usize, memory: &mut HashSet<Vec<u8>>) {
        if memory.contains(&s) {
            return;
        }
        memory.insert(s.clone());
        
        let s1: Vec<_> = s.iter().enumerate().map(|(i, &v)| if i % 2 == 1 { (v + a) % 10 } else { v }).collect();
        Self::rec(s1, a, b, memory);
        
        let mut s2 = (&s[b..]).to_vec();
        s2.extend_from_slice(&s[..b]);
        Self::rec(s2, a, b, memory);
    }
}
