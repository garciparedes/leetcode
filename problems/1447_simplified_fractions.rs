use std::collections::HashSet;
use std::fmt;
use std::cmp;
use std::cmp::Ordering;

impl Solution {
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        let mut ans = HashSet::new();
        for i in 2..=n {
            for j in 1..i {
                let fraction = Fraction::new(j, i).reduce();
                ans.insert(fraction);
            }
        }
        
        return ans
            .into_iter()
            .map(|fraction| fraction.to_string())
            .collect();
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    fn new(numerator: i32, denominator: i32) -> Self {
        Self {numerator: numerator, denominator: denominator}
    }
    
    fn reduce(self) -> Self {
        let divisor = gcd(self.numerator, self.denominator);
        if divisor > 1 {
            return Self::new(self.numerator / divisor, self.denominator / divisor);
        }
        return self;
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        Ordering::Less => gcd(a, b - a),
        Ordering::Equal => a,
        Ordering::Greater => gcd(a - b, b),
    }
}

