
impl Solution {
    pub fn valid_ip_address(ip: String) -> String {
        if Solution::valid_ipv4_address(&ip) {
            return String::from("IPv4");
        } 
        
        if Solution::valid_ipv6_address(&ip) {
            return String::from("IPv6");
        } 
        
        return String::from("Neither");
    }
    
    fn valid_ipv4_address(ip: &str) -> bool {
        let components: Vec<&str> = ip.split(".").collect();
        if !(components.len() == 4) {
            return false;
        }
        for component in components {
            if component.len() > 1 && component.starts_with("0") {
                return false;
            }
            match component.parse::<u8>() {
                Ok(number) => continue, 
                Error => return false,
            }
        }
        return true;
        
    }
    
    fn valid_ipv6_address(ip: &str) -> bool {
        let components: Vec<&str> = ip.split(":").collect();   
        if !(components.len() == 8) {
            return false;
        }
        if components[0].starts_with("0") {
            return false;
        }
        for component in components {
            if component.len() > 4 {
                return false;
            }
            
            if component.len() < 1 {
                return false;
            }    
            for c in component.chars() {
                let o = c as u32;
                if !(48 <= o && o <= 57) && !(65 <= o && o <= 70) && !(97 <= o && o <= 102) {
                    return false;
                }
            }   
        }
        return true;
    }
}
