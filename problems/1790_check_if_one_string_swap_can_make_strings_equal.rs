impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let (mut previous, mut swapped) = (None, false);
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 != c2 {
                if let Some((p1, p2)) = previous {
                    if swapped || c1 != p2 || c2 != p1 {
                        return false;
                    } else {
                        swapped = true;
                    }
                } else {
                    previous = Some((c1, c2));
                }
            }
        }
        return swapped == previous.is_some();
    }
}
