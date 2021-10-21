use std::marker::PhantomData;

use crate::search_space::{
    SearchSpace,
    GuidedSpace,
    TotalNeighborGeneration,
    Identifiable,
    ParetoDominanceSpace,
    ToSolution
};
use crate::search_combinator::SearchSpaceCombinator;

/// adds a depth field to a node
#[derive(Debug,Clone)]
pub struct DepthNode<N> {
    /// underlying node
    pub node: N,
    /// depth of the node
    pub depth: usize
}

/// pruning decorator: stores the best known solution and counts the number of prunings for statistics.
#[derive(Debug)]
pub struct NodeFeaturesCombinator<Space,B,G> {
    s: Space,
    /// vector of (bound, guide, depth, bool)
    node_information:Vec<(f64, f64, usize, bool)>,
    phantom_b:PhantomData<B>,
    phantom_g:PhantomData<G>,
}

impl<N,G,Space,B> GuidedSpace<DepthNode<N>,G> for NodeFeaturesCombinator<Space,B,G> 
where Space:GuidedSpace<N,G>
{
    fn guide(&mut self, n: &DepthNode<N>) -> G { self.s.guide(&n.node) }
}

impl<N,Sol,Space,B,G> ToSolution<DepthNode<N>,Sol> for NodeFeaturesCombinator<Space,B,G>
where Space:ToSolution<N,Sol> {
    fn solution(&mut self, node: &mut DepthNode<N>) -> Sol { self.s.solution(&mut node.node) }
}

impl<N,Space,B,G> SearchSpace<DepthNode<N>,B> for NodeFeaturesCombinator<Space,B,G>
where Space:SearchSpace<N,B>, B:serde::Serialize+PartialOrd+Clone
{
    fn initial(&mut self) -> DepthNode<N> { DepthNode { node:self.s.initial(), depth:0 } }

    fn bound(&mut self, n: &DepthNode<N>) -> B { self.s.bound(&n.node) }

    fn g_cost(&mut self, n: &DepthNode<N>) -> B { self.s.g_cost(&n.node) }

    fn goal(&mut self, n: &DepthNode<N>) -> bool { self.s.goal(&n.node) }

    fn restart(&mut self, msg: String) { self.s.restart(msg); }

    fn handle_new_best(&mut self, n: DepthNode<N>) -> DepthNode<N> {
        let depth:usize = n.depth;
        DepthNode { node: self.s.handle_new_best(n.node), depth }
    }

    fn stop_search(&mut self, _msg: String) { self.s.stop_search(_msg); }

    fn display_statistics(&self) { self.s.display_statistics(); }

    fn json_statistics(&self, json:&mut serde_json::Value) { self.s.json_statistics(json); }
}


impl<N, Space, B, G> TotalNeighborGeneration<DepthNode<N>> for NodeFeaturesCombinator<Space,B,G>
where 
    Space: TotalNeighborGeneration<N> + SearchSpace<N,B> + GuidedSpace<N,G>,
    B: Into<f64>,
    G: Into<f64>,
{

    fn neighbors(&mut self, n: &mut DepthNode<N>) -> Vec<DepthNode<N>> {
        self.s.neighbors(&mut n.node).into_iter().map(|child| {
            self.node_information.push((
                self.s.bound(&child).into(),
                self.s.guide(&child).into(),
                n.depth+1,
                self.s.goal(&child)
            ));
            DepthNode { node:child, depth:n.depth+1 }
        }).collect()
    }
}

impl<Space,B,G> SearchSpaceCombinator<Space> for NodeFeaturesCombinator<Space,B,G> {
    fn unwrap(&self) -> &Space { &self.s }
}


impl<Space,B,G> NodeFeaturesCombinator<Space,B,G> {
    /// builds the decorator around a search space
    pub fn new(s: Space) -> Self {
        Self {
            s,
            node_information: Vec::new(),
            phantom_b: PhantomData::default(),
            phantom_g: PhantomData::default()
        }
    }
}

impl<N, Id, Space,B,G> Identifiable<N, Id> for NodeFeaturesCombinator<Space,B,G>
where Space: Identifiable<N, Id>,
{
    fn id(&self, n: &mut N) -> Id { self.s.id(n) }
}


impl<N,Space,B,G> ParetoDominanceSpace<N> for NodeFeaturesCombinator<Space,B,G>
where Space: ParetoDominanceSpace<N>
{
    fn dominates(&self, a:&N, b:&N) -> bool { self.s.dominates(a,b) }
}