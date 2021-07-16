use std::marker::PhantomData;

use crate::search_space::{SearchSpace, GuidedSpace, TotalNeighborGeneration, PartialNeighborGeneration, Identifiable, ParetoDominanceSpace, ToSolution};
use crate::tree_search::decorators::helper::discrepancy::{DiscrepancyNode, DiscrepancyType};
use crate::search_decorator::SearchSpaceDecorator;

/**
 * Restrics the search tree by the children having positive remaining discrepancies.
 */
#[derive(Debug)]
pub struct LDSDecorator<Tree, D, G, B> {
    s: Tree,
    allowed_discrepancies: f64,
    discrepancy_type: D,
    phantom_g: PhantomData<G>,
    phantom_b: PhantomData<B>
}

impl<N,G,Tree,D,B> GuidedSpace<DiscrepancyNode<N>,G> for LDSDecorator<Tree, D, G, B>
where 
    Tree:GuidedSpace<N,G>
{
    fn guide(&mut self, n: &DiscrepancyNode<N>) -> G { self.s.guide(&n.node) }
}

impl<N, Space, D, G, B> TotalNeighborGeneration<DiscrepancyNode<N>> for LDSDecorator<Space, D, G, B>
where 
    Space: TotalNeighborGeneration<N>+GuidedSpace<N,G>+SearchSpace<N,B>,
    D: DiscrepancyType,
    G: Ord+Into<f64>+From<f64>
{
    fn neighbors(&mut self, n: &mut DiscrepancyNode<N>) -> Vec<DiscrepancyNode<N>> {
        if n.discrepancies > self.allowed_discrepancies {
            return Vec::new();
        }
        let mut tmp:Vec<DiscrepancyNode<N>> = self.discrepancy_type.compute_discrepancies(&mut self.s, n);
        // extracts and filters by discrepancies <= allowed
        let mut res = Vec::new();
        while !tmp.is_empty() {
            let n = tmp.pop().unwrap();
            if n.discrepancies <= self.allowed_discrepancies {
                res.push(n);
            }
        }
        res
    }
}


impl <Space, D, G, B, N, Sol> ToSolution<DiscrepancyNode<N>,Sol> for LDSDecorator<Space, D, G, B>
where
    Space: SearchSpace<N,B>+ToSolution<N,Sol>
{
    fn solution(&mut self, n: &mut DiscrepancyNode<N>) -> Sol { self.s.solution(&mut n.node) }
}


impl<N,Space,D,G,B> SearchSpace<DiscrepancyNode<N>,B> for LDSDecorator<Space, D, G, B>
where Space:SearchSpace<N,B>
{

    fn initial(&mut self) -> DiscrepancyNode<N> {
        DiscrepancyNode {
            node: self.s.initial(),
            discrepancies: 0.
        }
    }

    fn bound(&mut self, n: &DiscrepancyNode<N>) -> B { self.s.bound(&n.node) }

    fn g_cost(&mut self, n: &DiscrepancyNode<N>) -> B { self.s.g_cost(&n.node) }

    fn goal(&mut self, n: &DiscrepancyNode<N>) -> bool { self.s.goal(&n.node) }

    fn restart(&mut self, msg: String) {
        self.s.restart(msg);
    }

    fn handle_new_best(&mut self, n: DiscrepancyNode<N>) -> DiscrepancyNode<N> {
        DiscrepancyNode {
            node: self.s.handle_new_best(n.node),
            discrepancies: n.discrepancies
        }
    }

    fn stop_search(&mut self, _msg: String) {
        self.s.stop_search(_msg);
    }

    fn display_statistics(&self) {
        self.s.display_statistics();
    }

    fn export_statistics(&self, json:&mut serde_json::Value) {
        self.s.export_statistics(json);
    }
}

impl<Space, D, G, B> SearchSpaceDecorator<Space> for LDSDecorator<Space, D, G, B> {
    fn unwrap(&self) -> &Space { &self.s }
}

impl<Space, D, G, B> LDSDecorator<Space, D, G, B> {

    /** builds the decorator around a search space, the number of allowed discrepancies and
    a discrepancy policy */
    pub fn new(s: Space, allowed_discrepancies: f64, d:D) -> Self {
        Self {
            s,
            allowed_discrepancies,
            discrepancy_type: d,
            phantom_g: PhantomData,
            phantom_b: PhantomData
        }
    }
}

impl<N, B, Id, Space, D, G> Identifiable<DiscrepancyNode<N>, Id> for LDSDecorator<Space, D, G, B>
where
    Space: Identifiable<N, Id>,
{
    fn id(&self, n: &mut DiscrepancyNode<N>) -> Id { self.s.id(&mut n.node) }
}

impl<N,Space,D,G,B> ParetoDominanceSpace<DiscrepancyNode<N>> for LDSDecorator<Space, D, G, B>
where Space: ParetoDominanceSpace<N>
{
    fn dominates(&self, a:&DiscrepancyNode<N>, b:&DiscrepancyNode<N>) -> bool {
        self.s.dominates(&a.node,&b.node)
    }
}


impl<N,Tree,D,G,B> PartialNeighborGeneration<DiscrepancyNode<N>> for LDSDecorator<Tree, D, G, B>
where
    Tree: PartialNeighborGeneration<N>
{
    fn next_neighbor(&mut self, _node: &mut DiscrepancyNode<N>) -> Option<DiscrepancyNode<N>> {
        panic!("not implemented");
    }
}