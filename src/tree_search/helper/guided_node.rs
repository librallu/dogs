use std::cmp::Ordering;

/**
implements a guided node.
 - implements the Ord trait for nodes using the guide method from the search space)
 - fast guidance computation if the underlying search-space does not explicitely stores the guide
*/
#[derive(Debug)]
pub struct GuidedNode<N, G:Ord> {
    /// underlying node
    pub node: N,
    /// guide of the node
    pub guide: G,
}

impl<N, G:Ord> GuidedNode<N, G> {
    /**
    builds a GuidedNode from the node and its guide value
    */
    pub fn new(n:N, g:G) -> Self { GuidedNode { node:n, guide:g } }
}

impl<N, G:Ord> Ord for GuidedNode<N, G> {
    fn cmp(&self, other: &Self) -> Ordering { self.guide.cmp(&other.guide) }
}

impl<N, G:Ord> PartialOrd for GuidedNode<N, G> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl<N, G:Ord> PartialEq for GuidedNode<N, G> {
    fn eq(&self, other: &Self) -> bool { self.guide.eq(&other.guide) }
}

impl<N, G:Ord> Eq for GuidedNode<N, G> {}

