use std::collections::HashSet;

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let expression: Vec<char> = expression.chars().collect();
        let tokenized = Token::tokenize(&expression);        
        let expanded = tokenized.expand();

        let mut ans: Vec<_> = expanded.into_iter().collect();
        ans.sort_unstable();
        
        return ans;
    }
}

#[derive(Debug)]
enum Token {
    Val(char),
    Union(Vec<Box<Token>>),
    Concat(Vec<Box<Token>>),
}

impl Token {
    
    fn tokenize(expr: &[char]) -> Self {
        let mut tokens = Vec::new();
        let mut union = Vec::new();
        let mut i = 0;
        let mut depth = 0;
        for j in 0..expr.len() {
            if depth > 0 {
                if expr[j] == '{' {
                    depth += 1;
                } else if expr[j] == '}' {
                    depth -= 1;
                    if depth == 0 {
                        let part = Self::tokenize(&expr[i..j]);
                        union.push(Box::new(part));
                    }
                }
            } else {
                if expr[j] == '{' {
                    depth += 1;
                    i = j + 1;
                    continue;
                } else if expr[j] == ',' {
                    if union.len() == 1 {
                        tokens.push(union.pop().unwrap());
                    } else {
                        tokens.push(Box::new(Self::Concat(union)));
                    }
                    union = Vec::new();
                } else {
                    union.push(Box::new(Self::Val(expr[j])));    
                }            
            }
        }
        if !union.is_empty() {
            if union.len() == 1 {
                tokens.push(union.pop().unwrap());
            } else {
                tokens.push(Box::new(Self::Concat(union)));
            }
        }
        if tokens.len() == 1 {
            return *tokens.pop().unwrap();
        }
        return Self::Union(tokens);
    }
    
    fn expand(&self) -> HashSet<String> {
        match self {
            Self::Union(parts) => {
                let mut ans = HashSet::new();
                for part in parts {
                    for expansion in part.expand() {
                        ans.insert(expansion);
                    }
                }
                return ans;
            },
            Self::Concat(parts) => {
                if parts.is_empty() {
                    return HashSet::new();
                }
                
                let mut expanded_parts: Vec<_> = parts.into_iter().map(|part| part.expand()).collect();
                let mut ans = expanded_parts.pop().unwrap();
                for part in expanded_parts.into_iter().rev() {
                    let mut tmp = HashSet::new();
                    for b in &ans {
                        for a in &part {
                            tmp.insert(format!("{}{}", a, b));
                        }    
                    }
                    ans = tmp;
                }
                return ans;
            },
            Self::Val(v) => vec![v.to_string()].into_iter().collect(),
        }
    }
}




