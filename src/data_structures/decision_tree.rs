use std::rc::Rc;

/**
Implements a data-structure that maintains decisions taken in the tree. It helps avoiding
having to copy a Vector in each node to maintain the solution value.
Using this data-structure works great on large instances and large branching factors.
*/
#[derive(Debug)]
pub struct DecisionTree<D> {
    pub d:D,
    pub parent:Option<Rc<DecisionTree<D>>>,
}

impl<D> DecisionTree<D> where D:Clone {
    /**
     creates a new decision tree (only a root node)
    */
    pub fn new(d:D) -> Rc<Self> {
        Rc::new(Self {
            d,
            parent: None,
        })
    }

    /**
     * returns the decision held by the current node
     */
    pub fn decision(n:&Rc<Self>) -> &D {
        &n.as_ref().d
    }

    /**
    * creates a new child and point it to the current decision tree
    */
    pub fn add_child(n:&Rc<Self>, d:D) -> Rc<Self> {
        Rc::new(Self {
            d,
            parent:Some(n.clone())
        })
    }

    /**
    returns all decisions taken so far from the root to the current node
    */
    pub fn decisions_from_root(n:&Rc<Self>) -> Vec<D> {
        let mut res:Vec<D> = Vec::new();
        let mut c = n.clone();
        loop {
            res.push(DecisionTree::decision(&c).clone());
            let c2;
            match &c.as_ref().parent {
                None => {
                    res.reverse();
                    return res;
                },
                Some(p) => {
                    c2 = p.clone();
                }
            }
            c = c2;
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
    fn create_root() {
        let tree = DecisionTree::new(0);
        assert_eq!(*DecisionTree::decision(&tree), 0);
    }

    #[test]
    fn create_child() {
        let a = DecisionTree::new(0);
        let b = DecisionTree::add_child(&a, 1);
        let c = DecisionTree::add_child(&b, 42);
        assert_eq!(DecisionTree::decisions_from_root(&c), vec![0,1,42]);
    }

    #[test]
    fn multiple_children() {
        let node_a = DecisionTree::new(0);
        let node_b = DecisionTree::add_child(&node_a, 1);
        let node_c = DecisionTree::add_child(&node_b, 2);
        let node_d = DecisionTree::add_child(&node_a, 3);
        let node_e = DecisionTree::add_child(&node_d, 4);
        assert_eq!(DecisionTree::decisions_from_root(&node_c), vec![0,1,2]);
        assert_eq!(DecisionTree::decisions_from_root(&node_e), vec![0,3,4]);
    }
}

