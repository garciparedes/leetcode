use std::collections::HashMap;


#[derive(Debug)]
struct Node {
    initial: bool,
    children: HashMap<String, Box<Node>>,
}

impl Node {
    fn new(initial: bool) -> Self {
        Self { initial: initial, children: HashMap::new() }
    }
    
    fn add(&mut self, path: &str) {
        if !self.initial && self.children.is_empty() {
            return;
        }
        self.initial = false;
        if let Some(index) = path[1..].find('/') {
            let name = path[1..1 + index].to_string();
            self.children.entry(name).or_insert_with(|| Box::new(Self::new(true))).add(&path[1 + index..]);
        } else {
            let name = path[1..].to_string();
            self.children.insert(name, Box::new(Self::new(false)));
        }
    }
    
    fn paths(&self) -> Vec<String> {
        let mut ans = Vec::new();
        for (name, node) in &self.children {
            for path in node.paths() {
                ans.push(format!("/{}{}", name, path));
            }
        }
        if ans.is_empty() {
            ans.push(String::new());
        }
        return ans;
    }
    
}


impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        let mut tree = Node::new(true);
        for path in &folder {
            tree.add(path);
        }        
        
        return tree.paths();
    }
}
