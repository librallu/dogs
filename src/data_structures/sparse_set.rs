/**
 implements a sparse set data-structure.
 this structure is efficient to remove all but one values, but is costly in memory.
 if *n* is the number elements and *m* the number of subsets in the set this data-structure has the following complexities:
 - memory: O(n+m)
 - insertion: O(1)
 - remove: O(1)
 - contains: O(1)
 - remove all but one: O(1)
*/
#[derive(Debug)]
pub struct SparseSet {
    /// list of (unsorted) values
    dense: Vec<usize>,
    /// sparse[i] = v <=> dense[v] = i
    sparse: Vec<usize>,
    /// maximum number of elements
    nb_max: usize,
    /// number of elements in the SparseSet
    n: usize,
}


impl SparseSet {
    pub fn new(nb_max: usize) -> Self {
        Self {
            dense: vec![usize::MAX;nb_max],
            sparse: vec![usize::MAX;nb_max],
            nb_max,
            n: 0,
        }
    }

    /**
    returns the number of elements in the set
    */
    pub fn len(&self) -> usize {
        self.n
    }

    pub fn nth(&self, i:usize) -> usize {
        debug_assert!(i<self.n);
        self.dense[i]
    }

    /** true iff e ∈ Set */
    pub fn contains(&self, e:usize) -> bool {
        self.sparse[e] < self.n
    }

    /** inserts e into the set */
    pub fn insert(&mut self, e:usize) {
        debug_assert!(!self.contains(e));
        debug_assert!(e < self.nb_max);
        self.sparse[e] = self.n;
        self.dense[self.n] = e;
        self.n += 1;
    }

    pub fn remove(&mut self, e:usize) {
        debug_assert!(self.contains(e));
        // put the last element at the position of e
        self.n -= 1;
        self.dense[self.sparse[e]] = self.dense[self.n];
        self.sparse[self.dense[self.n]] = self.sparse[e];
        self.sparse[e] = usize::MAX;
    }

    pub fn remove_all_but_one(&mut self, e:usize) {
        // put e at the first position
        self.sparse[self.dense[0]] = usize::MAX;
        self.dense[0] = e;
        self.sparse[e] = 0;
        self.n = 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn constructor() {
        let set = SparseSet::new(10);
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn insert() {
        let mut set = SparseSet::new(10);
        set.insert(5);
        set.insert(7);
        set.insert(9);
        assert_eq!(set.contains(5), true);
        assert_eq!(set.contains(7), true);
        assert_eq!(set.contains(9), true);
        assert_eq!(set.contains(0), false);
        assert_eq!(set.contains(1), false);
        assert_eq!(set.contains(2), false);
        assert_eq!(set.contains(3), false);
    }

    #[test]
    fn remove() {
        let mut set = SparseSet::new(10);
        set.insert(5);
        set.insert(7);
        assert_eq!(set.contains(5), true);
        assert_eq!(set.contains(7), true);
        assert_eq!(set.contains(3), false);
        set.remove(5);
        assert_eq!(set.contains(5), false);
        assert_eq!(set.contains(7), true);
        assert_eq!(set.contains(3), false);
    }

    #[test]
    fn removeallbutone() {
        let mut set = SparseSet::new(10);
        set.insert(5);
        set.insert(7);
        set.insert(3);
        assert_eq!(set.contains(5), true);
        assert_eq!(set.contains(7), true);
        assert_eq!(set.contains(3), true);
        set.remove_all_but_one(3);
        assert_eq!(set.contains(5), false);
        assert_eq!(set.contains(7), false);
        assert_eq!(set.contains(3), true);
    }

}

