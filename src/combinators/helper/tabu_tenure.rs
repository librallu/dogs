use std::hash::Hash;

use fxhash::FxHashSet;

/** defines the behavior of a tabu tenure component. */
pub trait TabuTenure<Node, Decision:Hash+Eq> {

    /** insert a decision d to the tabu list (and given the resulting node n) */
    fn insert(&mut self, n:&Node, d:Decision);

    /** true iff the tabu tenure contains the decision (and given the resulting node n) */
    fn contains(&mut self, n:&Node, d:&Decision) -> bool;
}

/** tabu tenure that maintains every decision taken so far (no forgetting). */
#[derive(Debug, Default)]
pub struct FullTabuTenure<Decision> {
    decisions:FxHashSet<Decision>,
}

impl<Node, Decision:Hash+Eq> TabuTenure<Node, Decision> for FullTabuTenure<Decision> {
    fn insert(&mut self, _n:&Node, d:Decision) {
        self.decisions.insert(d);
    }

    fn contains(&mut self, _n:&Node, d:&Decision) -> bool {
        self.decisions.contains(d)
    }
}

