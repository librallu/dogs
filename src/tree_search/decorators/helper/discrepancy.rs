use std::cmp::Reverse;

use crate::search_space::{GuidedSpace, TotalNeighborGeneration};

/**
 * Adds a discrepancy value within nodes
 */
#[derive(Clone)]
pub struct DiscrepancyNode<N> {
    pub node: N,
    pub discrepancies: f64
}


/**
 * Defines the discrepancy behaviour (constant, linear, *etc.*)
 */
pub trait DiscrepancyType {
    
    /**
     * given a root node and its neighbors, returns a discrepancy 
     */
    fn compute_discrepancies<S,N,G>(&mut self, s:&mut S, n:&mut DiscrepancyNode<N>) -> Vec<DiscrepancyNode<N>>
        where S:TotalNeighborGeneration<N>+GuidedSpace<N,G>, G:Ord+Into<f64>+From<f64>;
}


/**
 * Linear discrepancy. The best child gets 0, the second best 1, *etc.*
 */
pub struct LinearDiscrepancy {}

impl DiscrepancyType for LinearDiscrepancy {
    fn compute_discrepancies<S,N,G>(&mut self, s:&mut S, n:&mut DiscrepancyNode<N>) -> Vec<DiscrepancyNode<N>> 
    where S:TotalNeighborGeneration<N>+GuidedSpace<N,G>, G:Ord {
        let d:f64 = n.discrepancies;
        let mut neighbors:Vec<N> = s.neighbors(&mut n.node);
        neighbors.sort_by_key(|e| Reverse(s.guide(e)));
        let mut res:Vec<DiscrepancyNode<N>> = Vec::new();
        let mut i = 0;
        while !neighbors.is_empty() {
            let tmp = neighbors.pop().unwrap();
            res.push(DiscrepancyNode {node:tmp, discrepancies:d+(i as f64)});
            i += 1;
        }
        return res;
    }
}

impl LinearDiscrepancy {
    pub fn new() -> Self { Self {} }
}


/**
 * Constant discrepancy. The best child gets 0, the others 1
 */
pub struct ConstantDiscrepancy {}

impl DiscrepancyType for ConstantDiscrepancy {
    fn compute_discrepancies<S,N,G>(&mut self, s:&mut S, n:&mut DiscrepancyNode<N>) -> Vec<DiscrepancyNode<N>> 
    where S:TotalNeighborGeneration<N>+GuidedSpace<N,G>, G:Ord {
        let d:f64 = n.discrepancies;
        let mut neighbors:Vec<N> = s.neighbors(&mut n.node);
        neighbors.sort_by_key(|e| Reverse(s.guide(e)));
        let mut res:Vec<DiscrepancyNode<N>> = Vec::new();
        let mut i = 0;
        while !neighbors.is_empty() {
            let tmp = neighbors.pop().unwrap();
            match i {
                0 => res.push(DiscrepancyNode {node:tmp, discrepancies:d}),
                _ => res.push(DiscrepancyNode {node:tmp, discrepancies:d+1.}),
            }
            
            i += 1;
        }
        return res;
    }
}

impl ConstantDiscrepancy {
    pub fn new() -> Self { Self {} }
}



/**
 * Ratio to best discrepancy. The best child (guide value of g0) gets 0, the second best gets (g1-g0)/g1 etc.
 * If g1 = 0, the discrepancy is 0. 
 */
pub struct RatioToBestDiscrepancy {}

impl DiscrepancyType for RatioToBestDiscrepancy {
    fn compute_discrepancies<S,N,G>(&mut self, s:&mut S, n:&mut DiscrepancyNode<N>) -> Vec<DiscrepancyNode<N>> 
    where S:TotalNeighborGeneration<N>+GuidedSpace<N,G>, G:Ord+Into<f64>+From<f64> {
        let d:f64 = n.discrepancies;
        let mut neighbors:Vec<N> = s.neighbors(&mut n.node);
        if neighbors.is_empty() {
            return Vec::new();
        }
        // invariant: neighbors contains at least 1 element
        neighbors.sort_by_key(|e| Reverse(s.guide(e)));
        let n:N = neighbors.pop().unwrap();
        let g0:f64 = s.guide(&n).into();
        let mut res:Vec<DiscrepancyNode<N>> = Vec::new();
        res.push(DiscrepancyNode {node:n, discrepancies:d});
        while !neighbors.is_empty() {  // extracts other neighbors and updates their discrepancies
            let n:N = neighbors.pop().unwrap();
            let gn:f64 = s.guide(&n).into();
            let mut discrepancy_increment = 0.;
            if gn > 0. {
                discrepancy_increment = (gn-g0)/gn;
            }
            res.push(DiscrepancyNode {node:n, discrepancies:d+discrepancy_increment});
        }
        return res;
    }
}

impl RatioToBestDiscrepancy {
    pub fn new() -> Self { Self {} }
}