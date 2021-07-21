use std::rc::Rc;

/**
Implements a data-structure that allows to perform a lazy computation.
This object is similar to a [https://doc.rust-lang.org/std/borrow/enum.Cow.html](Cow).
Initially, it stores a reference with a "parent" object. When a get or get_mut is called,
a computation occurs if needed.
*/
#[derive(Debug, Clone)]
pub enum LazyComputable<T:Clone> {
    /// reference to the "parent" element
    Ref(Rc<T>),
    /// reference to "itself"
    Computed(Rc<T>)
}

/**
Lazy clonable structure. Its content is cloned only needed from its parent.
*/
#[derive(Debug)]
pub struct LazyClonable<T:Clone> {
    content: LazyClonableContent<T>
}

impl<T:Clone> LazyClonable<T> {
    /**
    builds the lazy clonable object (the first one is computed by dafault)
    */
    pub fn new(t:T) -> Self {
        Self {
            content: LazyClonableContent::Computed(Rc::new(t))
        }
    }

    /**
    if already computed, get nothing, otherwise clone from the parent and become "computed"
    */
    pub fn lazy_get(&mut self) -> Rc<T> {
        match &self.content {
            LazyClonableContent::Ref(r) => {
                // if not "computed" yet: update the content to "computed"
                let r2 = Rc::new(r.as_ref().clone());
                self.content = LazyClonableContent::Computed(r2.clone());
                r2
            }, // otherwise, just return the "computed" reference
            LazyClonableContent::Computed(r) => { r.clone() }
        }
    }

    /**
    return a reference to the parent in any case
    */
    pub fn lazy_clone(&self) -> Self {
        Self {
            content: LazyClonableContent::Ref(
                match &self.content {
                    LazyClonableContent::Ref(r) => r.clone(),
                    LazyClonableContent::Computed(r) => r.clone()
                }
            )
        }
    }

    /**
    true iff the object has already been cloned.
    */
    pub fn is_cloned(&self) -> bool {
        match &self.content {
            LazyClonableContent::Ref(_) => false,
            LazyClonableContent::Computed(_) => true
        }
    }
}

impl<T:Clone> Clone for LazyClonable<T> {
    fn clone(&self) -> Self {
        self.lazy_clone()
    }
}


/*
 * UNIT TESTING
 */
#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn simple_construct() {
        let mut a = LazyClonable::new(42);
        // a.is_cloned() should be true because it is passed the initial value
        assert!(a.is_cloned());
        assert_eq!(*(a.lazy_get().as_ref()), 42);
    }

    #[test]
    fn simple_clone() {
        let a = LazyClonable::new(42);
        // a.is_cloned() should be true because it is passed the initial value
        assert!(a.is_cloned());
        let mut b = a.lazy_clone();
        assert!(!b.is_cloned());
        assert_eq!(*b.lazy_get().as_ref(), 42);
        assert!(b.is_cloned());
    }
}

