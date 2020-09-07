use std::cmp;

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut ans = Vec::new();
        
        let mut search_word: Vec<_> = search_word.chars().collect();
        let mut products: Vec<_> = products
            .into_iter()
            .map(|product| product.chars().collect::<Vec<char>>())
            .collect();
        
        for i in 0..search_word.len() {
            let mut j = 0;
            while j < products.len() {
                if (i >= products[j].len()) || (products[j][i] != search_word[i]) {
                    products.remove(j);
                } else {
                    j += 1;
                }
            } 
            products.sort_unstable();
            let mut tmp = Vec::new();
            for j in 0..cmp::min(3, products.len()) {
                let product = products[j].iter().collect::<String>();
                tmp.push(product);
            }
            ans.push(tmp);
        }
        return ans;
    }
}
