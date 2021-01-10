use crate::searchspace::{SearchSpace, GuidedSpace, PrefixEquivalenceTree, SearchTree, TotalChildrenExpansion};

pub struct PruningDecorator<Tree, B> {
    s: Tree,
    best_val: Option<B>,
    nb_prunings: u64
}

impl<N,G,Tree,B> GuidedSpace<N,G> for PruningDecorator<Tree, B> 
where Tree:GuidedSpace<N,G>
{
    fn guide(&mut self, n: &N) -> G {
        return self.s.guide(n);
    }
}

impl<N,Sol,Tree,B> SearchSpace<N,Sol> for PruningDecorator<Tree,B>
where Tree:SearchSpace<N,Sol>, B:serde::Serialize
{
    fn solution(&mut self, n: &N) -> Sol {
        return self.s.solution(n);
    }

    fn restart(&mut self, msg: String) {
        self.s.restart(msg);
    }

    fn handle_new_best(&mut self, n: &N) {
        self.s.handle_new_best(n);
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

impl<N, B, Tree> SearchTree<N,B> for PruningDecorator<Tree, B>
where
    B: PartialOrd+Copy,
    Tree: SearchTree<N, B>
{
    fn root(&mut self) -> N {
        return self.s.root();
    }

    fn bound(&mut self, n: &N) -> B {
        return self.s.bound(n);
    }


    fn goal(&mut self, n: &N) -> bool {
        let res = self.s.goal(n);
        if res {
            let eval_n = self.s.bound(n);
            match self.best_val {  // check if the best-known should be updated
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
}


impl<N, Tree, B> TotalChildrenExpansion<N> for PruningDecorator<Tree,B>
where 
    Tree: TotalChildrenExpansion<N>+SearchTree<N,B>,
    B: PartialOrd+Copy,
{

    fn children(&mut self, n: &mut N) -> Vec<N> where Tree:SearchTree<N, B> {
        match self.best_val {
            None => { return self.s.children(n); }
            Some(best_v) => {
                let mut res = Vec::new();
                let mut children = self.s.children(n);
                let children_size = children.len();
                while !children.is_empty() {
                    let child = children.pop().unwrap();
                    if self.s.bound(&child) < best_v {
                        res.push(child);
                    }
                }
                self.nb_prunings += children_size as u64 - res.len() as u64;
                return res;
            }
        }
    }
}


impl<Tree, B> PruningDecorator<Tree, B> {
    pub fn unwrap(&self) -> &Tree {
        return &self.s;
    }

    pub fn new(s: Tree) -> Self {
        Self {s: s, best_val: None, nb_prunings: 0}
    }
}

impl<N, B, PE, Tree> PrefixEquivalenceTree<N, B, PE> for PruningDecorator<Tree, B>
where
    Tree: PrefixEquivalenceTree<N, B, PE>,
{
    fn get_pe(&self, n: &N) -> PE {
        return self.s.get_pe(n);
    }

    fn prefix_bound(&self, n: &N) -> B {
        return self.s.prefix_bound(n);
    }
}