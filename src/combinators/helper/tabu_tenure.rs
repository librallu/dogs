use std::hash::Hash;

use fxhash::FxHashSet;

/** defines the behavior of a tabu tenure component. */
pub trait TabuTenure<Decision:Hash+Eq> {

    /** insert a decision to the tabu list */
    fn insert(&mut self, d:Decision);

    /** true iff the tabu tenure contains the decision */
    fn contains(&self, d:&Decision) -> bool;
}

/** tabu tenure that maintains every decision taken so far (no forgetting). */
#[derive(Debug, Default)]
pub struct FullTabuTenure<Decision> {
    decisions:FxHashSet<Decision>,
}

impl<Decision:Hash+Eq> TabuTenure<Decision> for FullTabuTenure<Decision> {
    fn insert(&mut self, d:Decision) {
        self.decisions.insert(d);
    }

    fn contains(&self, d:&Decision) -> bool {
        self.decisions.contains(d)
    }
}

