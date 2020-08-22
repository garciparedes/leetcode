impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let s: Vec<char> = s.chars().collect();
        let mut cases = Vec::new();
        Self::rec(Vec::new(), &s, &mut cases);
        return cases;
    }
    
    fn rec(mut current: Vec<char>, target: &Vec<char>, cases: &mut Vec<String>) {
        if current.len() == target.len() {
            cases.push(current.into_iter().collect());
            return
        }
        
        let i = current.len();    
        let v = target[i];
        
        if target[i].is_alphabetic() {
            let mut current_upper = current.clone();
            current_upper.push(v.to_uppercase().to_string().chars().next().unwrap());
            Self::rec(current_upper, target, cases);
            
            current.push(v.to_lowercase().to_string().chars().next().unwrap());
            Self::rec(current, target, cases);
        } else {
            current.push(v);
            Self::rec(current, target, cases);
        }
    }
}
