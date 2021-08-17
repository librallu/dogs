use std::marker::PhantomData;
use serde::Serialize;

use crate::search_space::{
    SearchSpace,
    GuidedSpace,
    TotalNeighborGeneration,
    PartialNeighborGeneration,
    Identifiable,
    ParetoDominanceSpace,
    ToSolution,
    BoundedDistanceSpace,
};
use crate::search_combinator::SearchSpaceCombinator;

/**
guide_with_bound decorator: generates a guide that incorporates the bound and the guide dynamically.
*/
#[derive(Debug)]
pub struct GuideWithBoundCombinator<Space,N,B> {
    s: Space,
    avg_bound: Vec<f64>,
    avg_guide: Vec<f64>,
    nb_vals:   Vec<u64>,
    phantom_n: PhantomData<N>,
    phantom_b: PhantomData<B>,
}

impl<N,G,B,Space> GuidedSpace<N,G> for GuideWithBoundCombinator<Space,N,B> 
where 
    Space:GuidedSpace<N,G>+SearchSpace<N,B>+BoundedDistanceSpace<N>,
    G:Into<f64>+From<f64>,
    B:Into<i64>+From<i64>+Serialize+PartialOrd+Clone,
{
    fn guide(&mut self, n: &N) -> G {
        let distance_from_root = self.s.distance_from_root(n);
        let guide:f64 = self.s.guide(n).into();
        let bound:f64 = self.bound(n).into() as f64;
        // update guide and bound estimate
        self.nb_vals[distance_from_root] += 1;
        let learning_rate:f64 = 1./(self.nb_vals[distance_from_root] as f64);
        self.avg_bound[distance_from_root] += 
            (bound-self.avg_bound[distance_from_root])*learning_rate;
        self.avg_guide[distance_from_root] += 
            (guide-self.avg_guide[distance_from_root])*learning_rate;
        // ratio avg bound / avg guide (how much the bound is larger than the guide)
        let regularization_point = distance_from_root;
        let regularization = self.avg_bound[regularization_point]/self.avg_guide[regularization_point];
        let alpha = self.s.root_distance_ratio(n);
        // return guide
        G::from(alpha*bound + (1.-alpha)*regularization*guide)
    }
}

impl<N,Sol,Space,B> ToSolution<N,Sol> for GuideWithBoundCombinator<Space,N,B>
where Space:ToSolution<N,Sol> {
    fn solution(&mut self, node: &mut N) -> Sol { self.s.solution(node) }
}

impl<N,Space,B> SearchSpace<N,B> for GuideWithBoundCombinator<Space,N,B>
where Space:SearchSpace<N,B>+BoundedDistanceSpace<N>, B:serde::Serialize+PartialOrd+Clone
{
    fn initial(&mut self) -> N { self.s.initial() }

    fn bound(&mut self, n: &N) -> B { self.s.bound(n) }

    fn g_cost(&mut self, n: &N) -> B { self.s.g_cost(n) }

    fn goal(&mut self, n: &N) -> bool { self.s.goal(n) }

    fn restart(&mut self, msg: String) {
        self.s.restart(msg);
    }

    fn handle_new_best(&mut self, n: N) -> N {
        // println!();
        // for i in 0..self.s.maximum_root_distance() {
        //     print!("{:.2} ", self.avg_bound[i]/self.avg_guide[i]);
        // }
        // println!();
        // println!("\tbound:\t{:?}", self.avg_bound);
        // println!("\tguide:\t{:?}", self.avg_guide);
        // println!("\tnb v :\t{:?}", self.nb_vals);
        self.s.handle_new_best(n)
    }

    fn stop_search(&mut self, _msg: String) {
        self.s.stop_search(_msg);
    }

    fn display_statistics(&self) {
        self.s.display_statistics();
    }

    fn json_statistics(&self, json:&mut serde_json::Value) {
        self.s.json_statistics(json);
    }
}


impl<N, Space,B> TotalNeighborGeneration<N> for GuideWithBoundCombinator<Space,N,B>
where 
    Space: TotalNeighborGeneration<N>
{

    fn neighbors(&mut self, n: &mut N) -> Vec<N> { self.s.neighbors(n) }
}

impl<Space,N,B> SearchSpaceCombinator<Space> for GuideWithBoundCombinator<Space,N,B> {
    fn unwrap(&self) -> &Space { &self.s }
}


impl<Space,N,B> GuideWithBoundCombinator<Space,N,B> where Space:BoundedDistanceSpace<N> {
    /** builds the decorator around a search space */
    pub fn new(s: Space) -> Self {
        let max_depth = s.maximum_root_distance();
        Self {
            s,
            nb_vals:   vec![0  ; max_depth+1],
            avg_bound: vec![0. ; max_depth+1],
            avg_guide: vec![0. ; max_depth+1],
            phantom_n: PhantomData::default(),
            phantom_b: PhantomData::default(),
        }
    }
}

impl<N, Id, Space,B> Identifiable<N, Id> for GuideWithBoundCombinator<Space,N,B>
where
    Space: Identifiable<N, Id>,
{
    fn id(&self, n: &mut N) -> Id { self.s.id(n) }
}


impl<N,Space,B> ParetoDominanceSpace<N> for GuideWithBoundCombinator<Space,N,B>
where Space: ParetoDominanceSpace<N>
{
    fn dominates(&self, a:&N, b:&N) -> bool { self.s.dominates(a,b) }
}


impl<N, Space,B> PartialNeighborGeneration<N> for GuideWithBoundCombinator<Space,N,B>
where 
    Space: PartialNeighborGeneration<N>
{
    fn next_neighbor(&mut self, node: &mut N) -> Option<N> { self.s.next_neighbor(node) }
}

impl<N,Space,B> BoundedDistanceSpace<N> for GuideWithBoundCombinator<Space,N,B>
where Space:BoundedDistanceSpace<N> {
    fn maximum_root_distance(&self) -> usize { self.s.maximum_root_distance() }

    fn distance_from_root(&self, n:&N) -> usize { self.s.distance_from_root(n) }
}