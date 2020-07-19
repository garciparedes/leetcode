use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        emails
            .into_iter()
            .map(|email| {
                let mut parts: Vec<_> = email
                    .splitn(2, "@")
                    .collect();
                
                let local = parts[0]
                    .splitn(2, "+")
                    .next()
                    .unwrap()
                    .replace(".", "")
                    .to_string();
                
                let domain = parts[1]
                    .to_string();
                
                return (domain, local);
            })
            .collect::<HashSet<_>>()
            .len() as i32
    }
}
