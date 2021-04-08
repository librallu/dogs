use std::rc::Rc;

/**
Implements a data-structure that allows to perform a lazy copy. At the beginning, it only stores a
pointer towards the parent datastructure. When the lazyget method is called for the first time, the
datastructure is copied from the parent reference. For later calls, the lazyget method only returns
the existing copy.
*/

#[derive(Debug, Clone)]
pub enum LazyClonableContent<T> {
    Ref(Rc<T>),
    Computed(Rc<T>)
}

#[derive(Debug)]
pub struct LazyClonable<T> {
    content: LazyClonableContent<T>
}

impl<T:Clone> LazyClonable<T> {
    pub fn new(t:T) -> Self {
        Self {
            content: LazyClonableContent::Computed(Rc::new(t))
        }
    }

    pub fn lazyget(&mut self) -> Rc<T> {
        let res:Rc<T>;
        match &self.content {
            LazyClonableContent::Ref(r) => {
                res = Rc::new(r.as_ref().clone());
            },
            LazyClonableContent::Computed(r) => { res = r.clone() }
        }
        self.content = LazyClonableContent::Computed(res.clone());
        res
    }

    pub fn is_cloned(&self) -> bool {
        match &self.content {
            LazyClonableContent::Ref(_) => false,
            LazyClonableContent::Computed(_) => true
        }
    }
}

impl<T> Clone for LazyClonable<T> {
    fn clone(&self) -> Self {
        Self {
            content: LazyClonableContent::Ref(match &self.content {
                LazyClonableContent::Ref(r) => r.clone(),
                LazyClonableContent::Computed(r) => r.clone()
            })
        }
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
        assert_eq!(*(a.lazyget().as_ref()) == 42, true);
    }

    #[test]
    fn simple_clone() {
        let a = LazyClonable::new(42);
        // a.is_cloned() should be true because it is passed the initial value
        assert_eq!(a.is_cloned(), true);
        let mut b = a.clone();
        assert_eq!(b.is_cloned(), false);
        assert_eq!(*b.lazyget().as_ref(), 42);
        assert_eq!(b.is_cloned(), true);
    }
}

