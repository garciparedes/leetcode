use std::collections::HashSet;
use std::cmp::Ordering;
use std::cmp;

impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let queens: HashSet<_> = queens.into_iter().collect();
        let mut matches = Vec::new();
        for queen in queens.iter() {
            if Self::check_attack(&king, queen, &queens) {
                matches.push(queen.clone());  
            }
        }
        return matches;
    }
    
    fn check_attack(king: &Vec<i32>, queen: &Vec<i32>, queens: &HashSet<Vec<i32>>) -> bool {
        return match king[0].cmp(&queen[0]) {
            Ordering::Less => {
                match king[1].cmp(&queen[1]) {
                    Ordering::Less => {
                        for i in 1..cmp::min(queen[0] - king[0], queen[1] - king[1]) + 1 {
                            if queen[0] - i == king[0] && queen[1] - i == king[1] {
                                return true;
                            }
                            if queen[0] - i < king[0] || queen[1] - i < king[1] {
                                return false;
                            }
                            if queens.contains(&vec![queen[0] - i, queen[1] - i]) {
                                return false;
                            }
                        }
                        return false;
                    },
                    Ordering::Equal => {
                        for i in 1..(queen[0] - king[0]) + 1 {
                            if queen[0] - i == king[0] {
                                return true;
                            }
                            if queens.contains(&vec![queen[0] - i, queen[1]]) {
                                return false;
                            }
                        }
                        return false;
                    },
                    Ordering::Greater => {
                        for i in 1..cmp::min(queen[0] - king[0], king[1] - queen[1]) + 1 {
                            if queen[0] - i == king[0] && queen[1] + i == king[1] {
                                return true;
                            }
                            if queen[0] - i < king[0] || queen[1] + i > king[1] {
                                return false;
                            }
                            if queens.contains(&vec![queen[0] - i, queen[1] + i]) {
                                return false;
                            }
                        }
                        return false;
                    },
                }
            },
            Ordering::Equal => {
                match king[1].cmp(&queen[1]) {
                    Ordering::Less => {
                        for i in 1..(queen[1] - king[1] + 1) {
                            if queen[1] - i == king[1] {
                                return true;
                            }
                            if queens.contains(&vec![queen[0], queen[1] - i]) {
                                return false;
                            }
                        }
                        return false;
                    },
                    Ordering::Greater => {
                        for i in 1..(king[1] - queen[1] + 1) {
                            if queen[1] + i == king[1] {
                                return true;
                            }
                            if queens.contains(&vec![queen[0], queen[1] + i]) {
                                return false;
                            }
                        }
                        return false;
                    },
                    Ordering::Equal => panic!(),
                }
            },
            Ordering::Greater => {
                match king[1].cmp(&queen[1]) {
                    Ordering::Less => {
                        for i in 1..cmp::min(king[0] - queen[0], queen[1] - king[1]) + 1 {
                            if queen[0] + i == king[0] && queen[1] - i == king[1] {
                                return true;
                            }
                            if queen[0] + i > king[0] || queen[1] - i < king[1] {
                                return false;
                            }
                            if queens.contains(&vec![queen[0] + i, queen[1] - i]) {
                                return false;
                            }
                        }
                        return false;
                    },
                    Ordering::Equal => {
                        for i in 1..(king[0] - queen[0]) + 1 {
                            if queen[0] + i == king[0] {
                                return true;
                            }
                            if queens.contains(&vec![queen[0] + i, queen[1]]) {
                                return false;
                            }
                        }
                        return false;
                    },
                    Ordering::Greater => {
                        for i in 1..cmp::min(king[0] - queen[0], king[1] - queen[1]) + 1 {
                            if queen[0] + i == king[0] && queen[1] + i == king[1] {
                                return true;
                            }
                            if queen[0] + i > king[0] || queen[1] + i > king[1] {
                                return false;
                            }
                            if queens.contains(&vec![queen[0] + i, queen[1] + i]) {
                                return false;
                            }
                        }
                        return false;
                    },
                }
            },
        };
    }
}
