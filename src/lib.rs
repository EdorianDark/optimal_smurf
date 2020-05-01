use std::vec::Vec;

pub struct Problem {
    //let x_i \in {0,1}
    //maximize \sum v_i x_i
    v: Vec<i64>,
    //subject to \sum w_i x_i <= K
    w: Vec<i64>,
    k: i64,
}
#[derive(Debug)]
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

pub use branch_and_bound::bounding_solve;
mod branch_and_bound {
    use super::{Problem, Solution};

    #[derive(Clone, Debug)]
    struct Node {
        value: u64,
        room: u64,
        estimate: f64,
        contained: Vec<bool>, //according to opt_sequence
    }
    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
                && self.room == other.room
                && self.estimate == other.estimate
                && self.contained == other.contained
        }
    }
    impl Eq for Node {}

    impl Ord for Node {
        fn cmp(&self, other: &Node) -> std::cmp::Ordering {
            self.estimate.partial_cmp(&other.estimate).expect("a Nan")
        }
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Node {
        fn validate(&self, sequence: &Vec<usize>, p: &Problem) {
            let value: i64 = self
                .contained
                .iter()
                .zip(sequence.iter())
                .filter(|(b, _)| **b)
                .map(|(_, i)| p.v[*i as usize])
                .sum();

            assert_eq!(value as u64, self.value);
        }
    }

    fn find_optimal_sequence(problem: &Problem) -> Vec<usize> {
        assert_eq!(problem.v.len(), problem.w.len());

        let v_iter = problem.v.iter().map(|v| *v as f64);
        let w_iter = problem.w.iter().map(|w| *w as f64);
        let normalized = v_iter.zip(w_iter).map(|(v, w)| v / w);
        let mut values: Vec<_> = normalized.enumerate().collect();
        values.sort_by(|(_, a), (_, b)| b.partial_cmp(a).expect("a Nan"));
        values.iter().map(|(i, _)| i.clone()).collect()
    }

    fn new_node(p: &Problem) -> Node {
        Node {
            value: 0,
            room: p.k as u64,
            estimate: 0f64,
            contained: vec![],
        }
    }

    fn build_next_node(
        parent: &Node,
        sequence: &Vec<usize>,
        take: bool,
        p: &Problem,
    ) -> Option<Node> {
        assert!(p.v.len() <= sequence.len());
        if parent.contained.len() == sequence.len() {
            return None;
        }
        let mut node = parent.clone();
        if take {
            let index = node.contained.len();
            let pos = sequence[index];
            let w = p.w[pos] as u64;
            if node.room < w {
                return None;
            }
            node.room -= w;
            node.value += p.v[pos] as u64;
        }
        node.contained.push(take);
        node.estimate = estimate(&node, sequence, p);
        Some(node)
    }

    fn estimate(node: &Node, sequence: &Vec<usize>, p: &Problem) -> f64 {
        assert!(sequence.len() == p.w.len());
        assert!(node.contained.len() <= sequence.len());
        if node.contained.len() == sequence.len() {
            return node.value as f64;
        }
        let mut room = node.room;
        let mut index = node.contained.len();
        let mut value = node.value as f64;
        while room > 0 && index < sequence.len() {
            let pos = sequence[index];
            let w = p.w[pos] as u64;
            if w <= room {
                room -= p.w[pos] as u64;
                value += p.v[pos] as f64;
            } else {
                let fit = room as f64 / w as f64;
                let peace_value = p.v[pos] as f64 * fit;
                value += peace_value;
                room = 0;
            }
            index += 1;
        }
        value
    }

    pub fn bounding_solve(problem: Problem) -> Solution {
        let opt_sequence = find_optimal_sequence(&problem);
        let mut start = new_node(&problem);
        start.estimate = estimate(&start, &opt_sequence, &problem);

        let mut local_max = start.clone();
        let mut queue = std::collections::BinaryHeap::new();
        queue.push(start);
        while (local_max.value as f64) < queue.peek().unwrap().estimate {
            let node = queue.pop().unwrap();
            let next_true = build_next_node(&node, &opt_sequence, true, &problem);
            if let Some(n) = next_true {
                if local_max.value < n.value {
                    local_max = n.clone();
                }
                queue.push(n);
            }
            let next_false = build_next_node(&node, &opt_sequence, false, &problem);
            if let Some(n) = next_false {
                if local_max.value < n.value {
                    local_max = n.clone();
                }
                queue.push(n);
            }
        }

        local_max.validate(&opt_sequence, &problem);
        while local_max.contained.len() < opt_sequence.len() {
            local_max.contained.push(false);
        }

        assert_eq!(local_max.contained.len(), opt_sequence.len());

        let mut value = 0;
        let mut contained = vec![false; opt_sequence.len()];
        for (i, s) in opt_sequence.iter().enumerate() {
            if local_max.contained[i] {
                value += problem.v[*s];
                contained[*s] = true;
            }
        }
        assert_eq!(value as u64, local_max.value);

        Solution { value, contained }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        fn build_problem() -> Problem {
            let v = vec![1, 1, 2, 3];
            let w = vec![2, 3, 5, 1];
            let k = 5;
            crate::build_problem(v.clone(), w.clone(), k.clone())
        }

        #[test]
        fn test_find_optimal_sequence() {
            let p = build_problem();
            let s = find_optimal_sequence(&p);
            assert_eq!(s, vec![3, 0, 2, 1]);
        }

        #[test]
        fn test_estimate_equal() {
            let mut p = build_problem();
            p.k = 1;
            let s = find_optimal_sequence(&p);
            let n = new_node(&p);
            let e = estimate(&n, &s, &p);
            assert_eq!(e, 3f64);
        }
        #[test]
        fn test_estimate_approx() {
            let mut p = build_problem();
            p.k = 2;
            let s = find_optimal_sequence(&p);
            let n = new_node(&p);
            let e = estimate(&n, &s, &p);
            assert_eq!(e, 3.5f64);
        }
        #[test]
        fn test_next_node() {
            let p = build_problem();
            let s = find_optimal_sequence(&p);
            let n1 = new_node(&p);
            let n2 = build_next_node(&n1, &s, true, &p);
            let m2 = Node {
                value: 3,
                contained: vec![true],
                room: 4,
                estimate: 4.8f64,
            };
            assert_eq!(n2.unwrap(), m2);
            let n3 = build_next_node(&n1, &s, false, &p);
            let m3 = Node {
                value: 0,
                contained: vec![false],
                room: 5,
                estimate: 2.2f64,
            };
            assert_eq!(n3.unwrap(), m3);
        }
        #[test]
        fn test_bounding_solve() {
            let v = vec![5, 6, 3];
            let w = vec![4, 5, 2];
            let k = 9;
            let problem = crate::build_problem(v, w, k);
            let solved = bounding_solve(problem);
            dbg!(&solved);
            assert_eq!(solved.contained, vec![true, true, false]);
            assert_eq!(solved.value, 11);
        }
    }
}

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
