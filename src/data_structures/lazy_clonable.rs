use std::rc::Rc;

/**
Implements a data-structure that allows to perform a lazy copy. At the beginning,
it only stores a pointer towards the parent datastructure. When the lazy_get method
is called for the first time, the datastructure is copied from the parent reference.
For later calls, the lazyget method only returns the existing copy.
*/
#[derive(Debug, Clone)]
pub enum LazyClonableContent<T:Clone> {
    /// reference to the "parent" element
    Ref(Rc<T>),
    /// reference to "itself"
    Computed(Rc<T>)
}

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
        ////////////// V3
        match &self.content {
            LazyClonableContent::Ref(r) => {
                // if not "computed" yet: update the content to "computed"
                let r2 = Rc::new(r.as_ref().clone());
                self.content = LazyClonableContent::Computed(r2.clone());
                r2
            }, // otherwise, just return the "computed" reference
            LazyClonableContent::Computed(r) => { r.clone() }
        }
        ////////////// V2
        // // if not "computed" yet: update the content to "computed"
        // if let LazyClonableContent::Ref(r) = &self.content {
        //     self.content = LazyClonableContent::Computed(
        //         Rc::new(r.as_ref().clone())
        //     );
        // }
        // match &self.content {
        //     LazyClonableContent::Computed(v) => v.clone(),
        //     LazyClonableContent::Ref(_) => panic!("lazy_clonable:lazy_get: something went wrong!")
        // }
        /////////// V1
        // let res:Rc<T>;
        // match &self.content {
        //     LazyClonableContent::Ref(r) => { res = Rc::new(r.as_ref().clone());
        //     },
        //     LazyClonableContent::Computed(r) => { res = r.clone() }
        // }
        // self.content = LazyClonableContent::Computed(res.clone());
        // res
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
        assert_eq!(a.is_cloned(), true);
        assert_eq!(*(a.lazy_get().as_ref()) == 42, true);
    }

    #[test]
    fn simple_clone() {
        let a = LazyClonable::new(42);
        // a.is_cloned() should be true because it is passed the initial value
        assert_eq!(a.is_cloned(), true);
        let mut b = a.lazy_clone();
        assert_eq!(b.is_cloned(), false);
        assert_eq!(*b.lazy_get().as_ref(), 42);
        assert_eq!(b.is_cloned(), true);
    }
}

