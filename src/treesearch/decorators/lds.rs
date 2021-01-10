use crate::searchspace::{SearchSpace, GuidedSpace, TotalChildrenExpansion, PrefixEquivalenceTree, SearchTree};
use crate::treesearch::decorators::helper::discrepancy::{DiscrepancyNode, DiscrepancyType};
use std::marker::PhantomData;

/**
 * Restrics the search tree by the children having positive remaining discrepancies.
 */
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
    fn guide(&mut self, n: &DiscrepancyNode<N>) -> G {
        return self.s.guide(&n.node);
    }
}

impl<N, Tree, D, G, B> TotalChildrenExpansion<DiscrepancyNode<N>> for LDSDecorator<Tree, D, G, B>
where 
    Tree: TotalChildrenExpansion<N>+GuidedSpace<N,G>+SearchTree<N,B>,
    D: DiscrepancyType,
    G: Ord+Into<f64>+From<f64>
{
    fn children(&mut self, n: &mut DiscrepancyNode<N>) -> Vec<DiscrepancyNode<N>> {
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
        return res;
    }
}

impl<N,Sol,Tree,D,G,B> SearchSpace<DiscrepancyNode<N>,Sol> for LDSDecorator<Tree, D, G, B>
where Tree:SearchSpace<N,Sol>
{
    fn solution(&mut self, n: &DiscrepancyNode<N>) -> Sol {
        return self.s.solution(&n.node);
    }

    fn restart(&mut self, msg: String) {
        self.s.restart(msg);
    }

    fn handle_new_best(&mut self, n: &DiscrepancyNode<N>) {
        self.s.handle_new_best(&n.node);
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


impl<N, B, G, Tree, D> SearchTree<DiscrepancyNode<N>,B> for LDSDecorator<Tree, D, G, B>
where B:PartialOrd, G:Ord+Into<f64>+From<f64>, Tree:SearchTree<N,B>, D:DiscrepancyType {

    fn root(&mut self) -> DiscrepancyNode<N> {
        DiscrepancyNode {
            node: self.s.root(),
            discrepancies: 0.
        }
    }

    fn bound(&mut self, n: &DiscrepancyNode<N>) -> B {
        return self.s.bound(&n.node);
    }

    fn goal(&mut self, n: &DiscrepancyNode<N>) -> bool {
        return self.s.goal(&n.node);
    }

}

impl<Tree, D, G, B> LDSDecorator<Tree, D, G, B> {
    pub fn unwrap(&self) -> &Tree {
        return &self.s;
    }

    pub fn new(s: Tree, allowed_discrepancies: f64, d:D) -> Self {
        Self {
            s: s,
            allowed_discrepancies: allowed_discrepancies,
            discrepancy_type: d,
            phantom_g: PhantomData,
            phantom_b: PhantomData
        }
    }
}

impl<N, B, PE, Tree, D, G> PrefixEquivalenceTree<DiscrepancyNode<N>, B, PE> for LDSDecorator<Tree, D, G, B>
where
    Tree: PrefixEquivalenceTree<N, B, PE>,
{
    fn get_pe(&self, n: &DiscrepancyNode<N>) -> PE {
        return self.s.get_pe(&n.node);
    }

    fn prefix_bound(&self, n: &DiscrepancyNode<N>) -> B {
        return self.s.prefix_bound(&n.node);
    }
}