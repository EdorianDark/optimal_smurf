use std::vec::Vec;

pub struct Problem {
    //let x_i \in {0,1}
    //maximize \sum v_i x_i
    v: Vec<i64>,
    //subject to \sum w_i x_i <= K
    w: Vec<i64>,
    k: i64,
}

pub struct Solution {
    pub value: i64,
    pub contained: Vec<bool>,
}

pub fn build_problem(v: Vec<i64>, w: Vec<i64>, k: i64) -> Problem {
    assert_eq!(v.len(), w.len());
    Problem { v, w, k }
}

mod dynamic_programming {
    use super::{Problem, Solution};
    use std::cmp::max;

    struct OracleCache {
        data: Vec<Vec<(usize, i64)>>, //elements, (first index, capacity)
    }
    impl OracleCache {
        fn store(&mut self, elements: usize, capacity: usize, value: i64) {
            if elements >= self.data.len() {
                self.data.push(vec![(0, 0)]);
            }
            let last = self.data[elements].last().unwrap();
            assert!(last.1 <= value);
            if last.1 < value {
                self.data[elements].push((capacity, value))
            }
        }
        fn get(&self, capacety: i64, elements: i64) -> i64 {
            if elements < 0 {
                return 0;
            }
            assert!(capacety >= 0);
            let capacety = capacety as usize;
            let elements = elements as usize;
            for (index, value) in self.data[elements].iter() {
                if *index <= capacety {
                    return *value;
                }
            }
            panic!();
        }
        fn build(capacety: i64, w: &Vec<i64>, v: &Vec<i64>) -> OracleCache {
            let mut cache = OracleCache { data: vec![] };
            for elements in 0..w.len() {
                for capacety in 0..=capacety {
                    let value = oracle(capacety, elements as i64, &w, &v);
                    cache.store(elements, capacety as usize, value);
                }
            }
            cache
        }
    }

    // k capacety, j element
    fn oracle(k: i64, j: i64, w: &Vec<i64>, v: &Vec<i64>) -> i64 {
        if j < 0 {
            0
        } else if w[j as usize] <= k {
            let last_selection = oracle(k, j - 1, w, v);
            let new_selection = v[j as usize] + oracle(k - w[j as usize], j - 1, w, v);
            max(last_selection, new_selection)
        } else {
            oracle(k, j - 1, w, v)
        }
    }

    // k capacety, j element
    fn cached_oracle(k: i64, j: i64, w: &Vec<i64>, v: &Vec<i64>, cache: &OracleCache) -> i64 {
        if j < 0 {
            0
        } else if w[j as usize] <= k {
            let last_selection = cache.get(k, j - 1);
            let new_selection = v[j as usize] + cache.get(k - w[j as usize], j - 1);
            max(last_selection, new_selection)
        } else {
            oracle(k, j - 1, w, v)
        }
    }

    pub fn dynamic_solve(p: Problem) -> Solution {
        let cache = OracleCache::build(p.k, &p.w, &p.v);

        let mut k = p.k;
        let mut value = 0;
        let mut contained = Vec::<bool>::new();
        for i in (0..p.w.len()).rev() {
            let i = i as i64;
            let previous = cached_oracle(k, i - 1, &p.w, &p.v, &cache);
            let current = cached_oracle(k, i, &p.w, &p.v, &cache);
            if previous < current {
                contained.push(true);
                k -= p.w[i as usize];
                value += p.v[i as usize];
            } else {
                contained.push(false);
            }
        }
        contained.reverse();
        Solution { value, contained }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_oracle() {
            let v = vec![5, 6, 3];
            let w = vec![4, 5, 2];
            assert_eq!(oracle(0, -1, &w, &v), 0);
            assert_eq!(oracle(9, -1, &w, &v), 0);
            assert_eq!(oracle(3, 0, &w, &v), 0);
            assert_eq!(oracle(4, 0, &w, &v), 5);
            assert_eq!(oracle(9, 0, &w, &v), 5);
            assert_eq!(oracle(3, 1, &w, &v), 0);
            assert_eq!(oracle(4, 1, &w, &v), 5);
            assert_eq!(oracle(5, 1, &w, &v), 6);
            assert_eq!(oracle(9, 1, &w, &v), 11);
            assert_eq!(oracle(3, 2, &w, &v), 3);
            assert_eq!(oracle(8, 2, &w, &v), 9);
        }
        #[test]
        fn test_dynamic_solve() {
            let v = vec![5, 6, 3];
            let w = vec![4, 5, 2];
            let k = 9;
            let problem = crate::build_problem(v, w, k);
            let solved = dynamic_solve(problem);
            assert_eq!(solved.contained, vec![true, true, false]);
            assert_eq!(solved.value, 11);
        }
    }
}
pub use dynamic_programming::dynamic_solve;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_problem() {
        let v = vec![1, 2, 3];
        let w = vec![3, 5, 1];
        let k = 42;
        let p = build_problem(v.clone(), w.clone(), k.clone());
        assert_eq!(p.v, v);
        assert_eq!(p.w, w);
        assert_eq!(p.k, k);
    }
}
