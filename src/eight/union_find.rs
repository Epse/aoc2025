use std::mem;
use std::fmt::Debug;

use itertools::Itertools;

#[derive(Debug)]
pub struct UnionFind<T> {
    values: Vec<T>,
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

// TODO unchecked indexing perhaps?

impl<T> UnionFind<T> where T: PartialEq {
    /// Finds the idx of root node for idx
    fn find(&mut self, mut idx: usize) -> usize {
        while idx != self.parents[idx] {
            let parent_idx = self.parents[idx];
            self.parents[idx] = self.parents[parent_idx];
            idx = parent_idx;
        }
        idx
    }

    /// Returns whether or not something happened
    fn union(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.find(x);
        let mut y = self.find(y);

        if x == y {
            return false; // Already in the same set
        }

        // This just avoids duplicating the code
        if self.sizes[x] < self.sizes[y] {
            mem::swap(&mut x, &mut y);
        }

        self.parents[y] = x;
        self.sizes[x] = self.sizes[y] + self.sizes[x];

        true
    }

    pub fn from_values(values: Vec<T>) -> UnionFind<T> {
        UnionFind {
            sizes: vec![1usize; values.len()],
            parents: (0usize..values.len()).collect(),
            values: values
        }
    }
    
    /// How many fully disjoint sets are there?
    pub fn set_count(&self) -> usize {
        self.parents.iter()
            .enumerate()
            .filter(|(idx, parent)| idx == *parent)
            .count()
    }

    pub fn union_values(&mut self, x: &T, y: &T) -> Result<bool, &'static str> {
        let x = self.values
            .iter()
            .position(|v| *v == *x)
            .ok_or("X value not in data")?;
        let y = self.values.iter().position(|v| *v == *y).ok_or("Y value not in data")?;

        Ok(self.union(x, y))
    }
}

#[allow(dead_code)]
impl <T> UnionFind<T> {
    pub fn sizes(&self) -> &Vec<usize> {
        &self.sizes
    }

    pub fn parents(&self) -> &Vec<usize> {
        &self.parents
    }
}

impl <T> UnionFind<T> where T: Debug + std::clone::Clone {
    #[allow(dead_code)]
    pub fn print_values_grouped(&self) {
        let mut a = self.values
            .iter()
            .enumerate()
            .map(|(idx, val)| (self.parents[idx], val))
            .collect::<Vec<(usize, &T)>>();
        a.sort_by(|a, b| a.0.cmp(&b.0));

        for (group, chunk) in &a.iter().chunk_by(|a| a.0) {
                println!("{:?} -> {:?}", group, chunk.map(|x| x.1).cloned().collect::<Vec<T>>());
            }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creation() {
        let values = vec![1,2,3,4,5,6];
        let uf = UnionFind::from_values(values.clone());
        assert_eq!(values.len(), uf.values.len());
        assert_eq!(values.len(), uf.parents.len());
        assert_eq!(values.len(), uf.sizes.len());
    }

    #[test]
    fn basic_unioning() {
        let mut uf = UnionFind::from_values(vec![1,2,3,4,5,6]);
        assert_eq!(6, uf.set_count());
        uf.union_values(&1, &2).unwrap();
        assert_eq!(5, uf.set_count());
    }

    #[test]
    fn test_double_unioning() {
        let mut uf = UnionFind::from_values(vec![1,2,3,4,5,6]);
        assert_eq!(6, uf.set_count());
        uf.union_values(&1, &2).unwrap();
        uf.union_values(&1, &2).unwrap();
        assert_eq!(5, uf.set_count());
    }

    #[test]
    fn test_one_parent() {
        let mut uf = UnionFind::from_values(vec![1,2,3,4]);
        uf.union_values(&1, &2).unwrap();
        uf.union_values(&3, &4).unwrap();
        uf.union_values(&1, &3).unwrap();
        assert_eq!(1, uf.set_count());
        assert_eq!(vec![4,1,2,1], uf.sizes);
    }

    #[test]
    fn test_set_counts() {
        let mut uf = UnionFind::from_values(vec![1,2,3,4,5,6]);
        uf.union_values(&1, &2).unwrap();
        uf.union_values(&3, &4).unwrap();
        uf.union_values(&4, &5).unwrap();
        let mut counts = uf.sizes().clone();
        counts.sort();
        assert_eq!(vec![1usize, 1usize, 1usize, 1usize, 2usize, 3usize], *counts);
    }
}
