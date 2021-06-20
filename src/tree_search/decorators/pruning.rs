use crate::search_space::{SearchSpace, GuidedSpace, TotalNeighborGeneration, PartialNeighborGeneration, Identifiable, ParetoDominanceSpace, ToSolution};

pub struct PruningDecorator<Space, B> {
    s: Space,
    best_val: Option<B>,
    nb_prunings: u64
}

impl<N,G,Space,B> GuidedSpace<N,G> for PruningDecorator<Space, B> 
where Space:GuidedSpace<N,G>
{
    fn guide(&mut self, n: &N) -> G {
        return self.s.guide(n);
    }
}

impl<N,Sol,Space,B> ToSolution<N,Sol> for PruningDecorator<Space, B>
where Space:ToSolution<N,Sol> {
    fn solution(&mut self, node: &mut N) -> Sol {
        return self.s.solution(node);
    }
}

impl<N,Space,B> SearchSpace<N,B> for PruningDecorator<Space,B>
where Space:SearchSpace<N,B>, B:serde::Serialize+PartialOrd+Clone
{
    fn initial(&mut self) -> N {
        return self.s.initial();
    }

    fn bound(&mut self, n: &N) -> B {
        return self.s.bound(n);
    }

    fn g_cost(&mut self, n: &N) -> B {
        return self.s.g_cost(n);
    }

    fn goal(&mut self, n: &N) -> bool {
        let res = self.s.goal(n);
        if res {
            let eval_n = self.s.bound(n);
            match self.best_val.clone() {  // check if the best-known should be updated
                None => { self.best_val = Some(eval_n) }
                Some(v) => {
                    if eval_n < v {
                        self.best_val = Some(eval_n);
                    } 
                }
            }
        }
        return res;
    }

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
        println!("{:>25}{:>15}", "nb pruned", self.nb_prunings);
        println!();
        self.s.display_statistics();
    }

    fn export_statistics(&self, json:&mut serde_json::Value) {
        json["nb_pruned"] = serde_json::json!(self.nb_prunings);
        self.s.export_statistics(json);
    }
}


impl<N, Space, B> TotalNeighborGeneration<N> for PruningDecorator<Space,B>
where 
    Space: TotalNeighborGeneration<N> + SearchSpace<N, B>,
    B: PartialOrd+Copy,
{

    fn neighbors(&mut self, n: &mut N) -> Vec<N> {
        match self.best_val {
            None => { return self.s.neighbors(n); }
            Some(best_v) => {
                let mut res = Vec::new();
                let mut children = self.s.neighbors(n);
                let children_size = children.len();
                while !children.is_empty() {
                    let child = children.pop().unwrap();
                    if self.s.bound(&child) < best_v {
                        res.push(child);
                    }
                }
                // res.reverse();
                self.nb_prunings += children_size as u64 - res.len() as u64;
                return res;
            }
        }
    }
}


impl<Space, B> PruningDecorator<Space, B> {
    pub fn unwrap(&self) -> &Space {
        return &self.s;
    }

    pub fn new(s: Space) -> Self {
        Self {s: s, best_val: None, nb_prunings: 0}
    }
}

impl<N, B, Id, Space> Identifiable<N, Id> for PruningDecorator<Space, B>
where
    Space: Identifiable<N, Id>,
{
    fn id(&self, n: &N) -> Id {
        return self.s.id(n);
    }
}


impl<N,Space,B> ParetoDominanceSpace<N> for PruningDecorator<Space, B>
where Space: ParetoDominanceSpace<N>
{
    fn dominates(&self, a:&N, b:&N) -> bool {
        return self.s.dominates(a,b);
    }
}


impl<N, Space, B> PartialNeighborGeneration<N> for PruningDecorator<Space,B>
where 
    Space: PartialNeighborGeneration<N>+SearchSpace<N,B>,
    B: PartialOrd+Copy,
{
    fn next_neighbor(&mut self, node: &mut N) -> Option<N> {
        match self.s.next_neighbor(node) {
            None => { return None; },
            Some(c) => {
                match self.best_val {
                    None => { return Some(c); }
                    Some(best_v) => {
                        if self.s.bound(&c) < best_v {
                            return Some(c);
                        } else {
                            self.nb_prunings += 1;
                            return self.next_neighbor(node);
                        }
                    }
                }
                
            }
        }
    }
}