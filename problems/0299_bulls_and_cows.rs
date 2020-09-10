use std::cmp;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {        
        let (mut bulls, mut cows) = (0, 0);
        let mut secret_freqs = vec![0; 10];
        let mut guess_freqs = vec![0; 10];        
        for (a, b) in secret.chars().zip(guess.chars()) {
            if a == b {
                bulls += 1;
            }    
            secret_freqs[a as usize - 48] += 1;
            guess_freqs[b as usize - 48] += 1;
        }
        for i in 0..10 {
            cows += cmp::min(guess_freqs[i], secret_freqs[i])
        }
        cows -= bulls;
        return format!("{}A{}B", bulls, cows);
    }
}
