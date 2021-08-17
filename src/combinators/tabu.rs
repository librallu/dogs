use std::marker::PhantomData;

use crate::search_space::{DecisionSpace, GuidedSpace, Identifiable, ParetoDominanceSpace, PartialNeighborGeneration, SearchSpace, ToSolution, TotalNeighborGeneration};
use crate::search_combinator::SearchSpaceCombinator;

use crate::combinators::helper::tabu_tenure::TabuTenure;

/**
pruning decorator: stores the best known solution and counts the number of prunings for statistics.
*/
#[derive(Debug)]
pub struct TabuCombinator<Space, B, Tenure, D> {
    s: Space,
    tenure: Tenure,
    phantom_b: PhantomData<B>,
    phandom_d: PhantomData<D>,
}

impl<N,G,Space,B, Tenure, D> GuidedSpace<N,G> for TabuCombinator<Space, B, Tenure, D> 
where Space:GuidedSpace<N,G>
{
    fn guide(&mut self, n: &N) -> G { self.s.guide(n) }
}

impl<N,Sol,Space,B, Tenure, D> ToSolution<N,Sol> for TabuCombinator<Space, B, Tenure, D>
where Space:ToSolution<N,Sol> {
    fn solution(&mut self, node: &mut N) -> Sol { self.s.solution(node) }
}

impl<N,Space,B, Tenure, D> SearchSpace<N,B> for TabuCombinator<Space,B,Tenure, D>
where Space:SearchSpace<N,B>, B:serde::Serialize+PartialOrd+Clone
{
    fn initial(&mut self) -> N { self.s.initial() }

    fn bound(&mut self, n: &N) -> B { self.s.bound(n) }

    fn g_cost(&mut self, n: &N) -> B { self.s.g_cost(n) }

    fn goal(&mut self, n: &N) -> bool { self.s.goal(n) }

    fn restart(&mut self, msg: String) {
        self.s.restart(msg);
    }

    fn handle_new_best(&mut self, n: N) -> N {
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


impl<N, Space, B, Tenure, Decision> TotalNeighborGeneration<N> for TabuCombinator<Space,B,Tenure, Decision>
where 
    Space: TotalNeighborGeneration<N> + SearchSpace<N, B> + DecisionSpace<N,Decision>,
    B: PartialOrd+Copy,
    Decision: std::hash::Hash+Eq,
    Tenure: TabuTenure<Decision>,
    N: Clone,
{

    fn neighbors(&mut self, n: &mut N) -> Vec<N> {
        // add the decision to the tabu list
        match self.s.decision(n) {
            None => {},
            Some(d) => self.tenure.insert(d)
        }
        // remove neighbors that have their decision in the tabu tenure
        self.s.neighbors(n).iter().filter(|neigh| {
            match self.s.decision(neigh) {
                None => true,
                Some(d) => !self.tenure.contains(&d)
            }
        }).cloned().collect()
    }
}

impl<Space, B, Tenure, D> SearchSpaceCombinator<Space> for TabuCombinator<Space, B, Tenure, D> {
    fn unwrap(&self) -> &Space { &self.s }
}


impl<Space, B, Tenure, D> TabuCombinator<Space, B, Tenure, D> {
    /** builds the decorator around a search space */
    pub fn new(s: Space, tenure:Tenure) -> Self {
        Self {s, tenure, phantom_b: PhantomData::default(), phandom_d: PhantomData::default() }
    }
}

impl<N, B, Id, Space, Tenure, D> Identifiable<N, Id> for TabuCombinator<Space, B, Tenure, D>
where
    Space: Identifiable<N, Id>,
{
    fn id(&self, n: &mut N) -> Id { self.s.id(n) }
}


impl<N,Space,B, Tenure, D> ParetoDominanceSpace<N> for TabuCombinator<Space, B, Tenure, D>
where Space: ParetoDominanceSpace<N>
{
    fn dominates(&self, a:&N, b:&N) -> bool { self.s.dominates(a,b) }
}


impl<N, Space, B, Tenure, D> PartialNeighborGeneration<N> for TabuCombinator<Space,B, Tenure, D>
where 
    Space: PartialNeighborGeneration<N>+SearchSpace<N,B>,
    B: PartialOrd+Copy,
{
    fn next_neighbor(&mut self, _node: &mut N) -> Option<N> {
        // TODO iterate until a node has not its decision in the tenure
        todo!()
        // self.s.next_neighbor(node)
    }
}