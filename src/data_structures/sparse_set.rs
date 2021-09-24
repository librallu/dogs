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
    /**
    creates a new sparse set from its maximum size (nb_max)
    */
    pub fn new(nb_max: usize) -> Self {
        Self {
            dense: vec![usize::MAX;nb_max],
            sparse: vec![usize::MAX;nb_max],
            nb_max,
            n: 0,
        }
    }

    /**
    returns true iff the set is empty
    */
    pub fn is_empty(&self) -> bool { self.n == 0 }

    /**
    returns the number of elements in the set
    */
    pub fn len(&self) -> usize {
        self.n
    }

    /** returns the nth element of the set */
    pub fn nth(&self, i:usize) -> usize {
        debug_assert!(i<self.n);
        self.dense[i]
    }

    /** true iff e ∈ Set */
    pub fn contains(&self, e:usize) -> bool {
        self.sparse[e] < self.n
    }

    /** inserts e into the set. Returns true iff the element was missing and successfully inserted */
    pub fn insert(&mut self, e:usize) -> bool {
        debug_assert!(e < self.nb_max);
        if !self.contains(e) {
            self.sparse[e] = self.n;
            self.dense[self.n] = e;
            self.n += 1;
            return true;
        }
        false
    }

    /** removes e from the set */
    pub fn remove(&mut self, e:usize) {
        debug_assert!(self.contains(e));
        // put the last element at the position of e
        self.n -= 1;
        self.dense[self.sparse[e]] = self.dense[self.n];
        self.sparse[self.dense[self.n]] = self.sparse[e];
        self.sparse[e] = usize::MAX;
    }

    /** removes everything except e from the set */
    pub fn remove_all_but_one(&mut self, e:usize) {
        debug_assert!(self.contains(e));
        // put e at the first position
        self.sparse[self.dense[0]] = usize::MAX;
        self.dense[0] = e;
        self.sparse[e] = 0;
        self.n = 1;
    }

    /** returns an iterator */
    pub fn iter(&'_ self) -> SparseSetIterator<'_> {
        SparseSetIterator::new(self)
    }
}

/** Sparse set iterator */
#[derive(Debug)]
pub struct SparseSetIterator<'a> {
    set: &'a SparseSet,
    index: usize,
}

impl<'a> SparseSetIterator<'a> {
    fn new(set: &'a SparseSet) -> Self {
        Self { set, index:0 }
    }
}

impl<'a> Iterator for SparseSetIterator<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.index < self.set.len() {
            let res = self.set.dense[self.index];
            self.index += 1;
            Some(res)
        } else {
            None
        }
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
        assert!(set.contains(5));
        assert!(set.contains(7));
        assert!(set.contains(9));
        assert!(!set.contains(0));
        assert!(!set.contains(1));
        assert!(!set.contains(2));
        assert!(!set.contains(3));
    }

    #[test]
    fn remove() {
        let mut set = SparseSet::new(10);
        set.insert(5);
        set.insert(7);
        assert!(set.contains(5));
        assert!(set.contains(7));
        assert!(!set.contains(3));
        set.remove(5);
        assert!(set.contains(7));
        assert!(!set.contains(5));
        assert!(!set.contains(3));
    }

    #[test]
    fn removeallbutone() {
        let mut set = SparseSet::new(10);
        set.insert(5);
        set.insert(7);
        set.insert(3);
        assert!(set.contains(5));
        assert!(set.contains(7));
        assert!(set.contains(3));
        set.remove_all_but_one(3);
        assert!(!set.contains(5));
        assert!(!set.contains(7));
        assert!(set.contains(3));
    }

}

