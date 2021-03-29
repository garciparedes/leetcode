use std::collections::HashMap;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let knowledge: HashMap<_, _> = knowledge
            .into_iter()
            .map(|entry| (entry[0].clone(), entry[1].clone()))
            .collect();
        
        let mut key: Option<String> = None;
        let mut ans = String::new();
        for c in s.chars() {
            if let Some(content) = key.as_mut() {
                if c == ')' {
                    if let Some(tmp) = knowledge.get(content) {                  
                        ans.push_str(tmp);  
                    } else {
                        ans.push('?');
                    }
                    key = None;                
                } else {
                    content.push(c);    
                }            
            } else {
                if c == '(' {
                    key = Some(String::new());
                } else {
                    ans.push(c);    
                }
            }
        }
        return ans;
    }
}
