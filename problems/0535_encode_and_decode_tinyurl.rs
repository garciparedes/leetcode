use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


struct Codec {
	memory: Rc<RefCell<HashMap<String, String>>>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        return Codec { memory: Rc::new(RefCell::new(HashMap::new())) }
    }
	
    // Encodes a URL to a shortened URL.
    fn encode(&self, longURL: String) -> String {
        if !self.memory.borrow().contains_key(&longURL) {
            let key: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(6)
                .collect();
            self.memory.borrow_mut().insert(longURL.clone(), key);
        }
        return format!("http://tinyurl.com/{}", self.memory.borrow().get(&longURL).unwrap());
    }
	
    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String {
        for (key, val) in self.memory.borrow().iter() {
            if format!("http://tinyurl.com/{}", val) != shortURL {
                continue;
            }
            return String::from(key);
        }
        return String::new();
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */
