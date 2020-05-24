impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        return address.replace(".", "[.]");       
    }
}
