impl Solution {
    pub fn largest_time_from_digits(mut a: Vec<i32>) -> String {
        let mut maximum = (4, 4, 4, 4);
        let mut best = 0;
        for i in 0..4 {
            if a[i] > 2 {
                continue;
            }
            for j in 0..4 {
                if (i == j) || (a[i] == 2) && (a[j] > 3) {
                    continue;
                }
                for k in 0..4 {
                    if (k == i) || (k == j) || (a[k] > 5) {
                        continue;
                    }
                    for l in 0..4 {
                         if (l == i) || (l == j) || (l == k) {
                            continue;
                        }
                        let current = 36000 * a[i] + 3600 * a[j] + 60 * a[k] + a[l];
                        if  best > current {
                           continue;
                        }
                        best = current;
                        maximum = (i, j, k, l);
                    }
                }
            }
        }
        if maximum.0 > 3 {
            return String::new();
        }
        return format!("{}{}:{}{}", a[maximum.0], a[maximum.1], a[maximum.2], a[maximum.3]);
    }
}
