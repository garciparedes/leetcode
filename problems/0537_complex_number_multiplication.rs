impl Solution {
    pub fn complex_number_multiply(a: String, b: String) -> String {
        let a: Vec<i32> = Self::parse(a);
        let b: Vec<i32> = Self::parse(b);
        
        let (real, imaginary) = (a[0] * b[0] - a[1] * b[1], a[0] * b[1] + a[1] * b[0]); 
        return format!("{}+{}i", real, imaginary);
    }
    
    fn parse(raw: String) -> Vec<i32> {
         raw
            .split("+")
            .map(|x| x.replace("i", "").parse::<i32>().unwrap())
            .collect()
    }
}
