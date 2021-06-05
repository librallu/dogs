use std::cmp::Ordering;

/**
 * implements a guided node (implements the Ord trait for nodes using the guide method from the search space)
 */
pub struct GuidedNode<N, G:Ord> {
    pub node: N,
    pub guide: G,
}

impl<N, G:Ord> GuidedNode<N, G> {
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

