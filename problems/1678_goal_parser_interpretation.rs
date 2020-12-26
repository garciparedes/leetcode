impl Solution {
    pub fn interpret(command: String) -> String {
        let mut ans = String::new();
        let mut iterable = command.chars();
        while let Some(c1) = iterable.next() {
            match c1 {
                'G' => ans.push('G'),
                _ => {
                    if let Some(c2) = iterable.next() {
                        match (c1, c2) {
                            ('(', ')') => ans.push('o'),
                            _ => {
                                if let (Some(c3), Some(c4)) = (iterable.next(), iterable.next()) {
                                    match (c1, c2, c3, c4) {
                                        ('(', 'a', 'l', ')') => ans.push_str("al"),
                                        _ => panic!(),
                                    }
                                } else {
                                    panic!();
                                }
                            }
                        }
                    } else {
                        panic!();
                    }
                }
            }
        }
        return ans;
    
    }
}
