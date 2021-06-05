use std::collections::BTreeMap;
use std::fmt::Display;
use std::cell::RefCell;
use std::rc::{Weak, Rc};
use std::cmp::max;
use serde::{Serialize};

use crate::metric_logger::{Metric, MetricLogger};
use crate::search_space::{SearchSpace, GuidedSpace, TotalNeighborGeneration, PartialNeighborGeneration, Identifiable, ParetoDominanceSpace, ToSolution};


pub trait LifetimeEventListener<B> {
    fn on_destructevent(&mut self, b: &B);
    fn on_insertvevent(&mut self, b: &B);
}

pub struct LifetimeEventNode<N, B, C>
where C: LifetimeEventListener<B> {
    pub node: N,
    pub bound: B,
    pub lifetime_listener: Rc<RefCell<C>>,
    pub expanded: bool,
}

impl<N,B,C> Clone for LifetimeEventNode<N,B,C>
where N:Clone, B:Clone, C: LifetimeEventListener<B> {
    fn clone(&self) -> Self {
        self.lifetime_listener.borrow_mut().on_insertvevent(&self.bound);
        return Self {
            node: self.node.clone(),
            bound: self.bound.clone(),
            lifetime_listener: self.lifetime_listener.clone(),
            expanded: false,
        }
    }
}

impl<N, B, C> Drop for LifetimeEventNode<N, B, C> 
where C: LifetimeEventListener<B> {
    fn drop(&mut self) {
        if self.expanded {
            self.lifetime_listener.borrow_mut().on_destructevent(&self.bound);
        }
    }
}


/**
 * Maintains the node bound set for the BoundingCombinator
 */
pub struct BoundSet<B> {
    pub set: BTreeMap<B, usize>,
    pub global_bound: Option<B>,
    pub global_bound_display: Option<B>,
    pub logger: Weak<MetricLogger>,
    pub logging_id_bound: Option<usize>,
}

impl<B> BoundSet<B> where B:Ord+Display+Clone+Copy+Into<i64> {
    pub fn new(logger: Weak<MetricLogger>,) -> Self {
        Self {
            set: BTreeMap::new(),
            global_bound: None,
            global_bound_display: None,
            logger: logger,
            logging_id_bound: None,
        }
    }

    pub fn reset(&mut self) {
        self.set.clear();
    }

    pub fn update_global_bound_insert(&mut self, b:&B) {
        // if first node opened ever
        if self.global_bound == None {
            self.global_bound = Some(b.clone());
            if let Some(logger) = self.logger.upgrade() {
                if let Some(id) = self.logging_id_bound {
                    logger.update_metric(id, Metric::Int(b.clone().into()));
                    // logger.request_logging();
                }
            }
        }
    }

    pub fn insert(&mut self, b:&B) {
        // updates BoundSet
        match self.set.get_mut(&b) {
            None => {
                self.set.insert(b.clone(), 1);
            },
            Some(v) => {
                *v += 1;
            }
        }
        // updates global bound
        self.update_global_bound_insert(b);
    }

    pub fn remove(&mut self, b:&B) {
        // updates BoundingSet
        let mut to_remove:bool = false;
        if let Some(v) = self.set.get_mut(&b) {
            if *v > 1 {
                *v -= 1;
            } else if *v == 1 {
                to_remove = true;
            } else {
                panic!("[BoundingCombinator] existing entry with value 0 (should be >= 1)");
            }
        } else { // if no entry, panic
            panic!("[BoundingCombinator] removing a bound value not-existing");
        }
        if to_remove {
            self.set.remove(&b);
            // updates global bound
            let mut logging_required = false;
            match self.global_bound {
                None => panic!("removing a bound value not-existing global bound"),
                Some(v) => {
                    let previous_bound = self.global_bound.clone();
                    self.global_bound = match self.set.iter().next() {
                        None => None,
                        Some(v2) => {
                            Some(max(v, v2.0.clone()))
                        },
                    };
                    if previous_bound < self.global_bound {
                        logging_required = true;  // only logs if improving the bound
                    }
                }
            }
            if logging_required {
                if let Some(logger) = self.logger.upgrade() {
                    if let Some(id) = self.logging_id_bound {
                        let metric = match self.global_bound {
                            None => Metric::Text("infeasible".to_string()),
                            Some(v) => Metric::Int(v.clone().into()),
                        };
                        logger.update_metric(id, metric);
                        // logger.request_logging();
                    }
                }
            }
        }
    }
}

impl<B> LifetimeEventListener<B> for BoundSet<B> 
where B:Ord+Display+Clone+Copy+Into<i64> {
    fn on_destructevent(&mut self, bound: &B) {
        self.remove(bound);
    }

    fn on_insertvevent(&mut self, bound: &B) {
        self.insert(bound);
    }
}

/**
 * Registers the global dual bound
 *  - when a node is destructed: remove its bound of the pq and update the global bound
 *  - TODO when a node bound is updated: update the global bound
 */
pub struct BoundingDecorator<Space, B> {
    s: Space,
    bound_set: Rc<RefCell<BoundSet<B>>>,
}


impl<N,G,Space,B> GuidedSpace<LifetimeEventNode<N, B, BoundSet<B>>,G> for BoundingDecorator<Space, B>
where Space:GuidedSpace<N,G>, B:Display+Ord+Copy+Into<i64>
{
    fn guide(&mut self, n: &LifetimeEventNode<N, B, BoundSet<B>>) -> G {
        return self.s.guide(&n.node);
    }
}

impl <N,Sol,B,Space> ToSolution<LifetimeEventNode<N, B, BoundSet<B>>,Sol> for BoundingDecorator<Space, B>
where
    Space: SearchSpace<N,B>+ToSolution<N,Sol>,
    B:Ord+Display+Copy+Into<i64>
{
    fn solution(&mut self, n: &LifetimeEventNode<N, B, BoundSet<B>>) -> Sol {
        return self.s.solution(&n.node);
    }
}


impl<N,Space,B> SearchSpace<LifetimeEventNode<N, B, BoundSet<B>>,B> for BoundingDecorator<Space, B>
where
    N:Clone,
    Space:SearchSpace<N,B>,
    B:Display+Ord+Copy+Into<i64>+Serialize,
    // C:LifetimeEventListener<B>
{

    fn initial(&mut self) -> LifetimeEventNode<N, B, BoundSet<B>> {
        let initial = self.s.initial();
        let bound = self.s.bound(&initial);
        self.insert_bound(&bound);
        return LifetimeEventNode {
            node: initial,
            bound: bound,
            lifetime_listener: self.bound_set.clone(),
            expanded: false,
        };
    }

    fn bound(&mut self, n: &LifetimeEventNode<N, B, BoundSet<B>>) -> B {
        return self.s.bound(&n.node);
    }

    fn goal(&mut self, n: &LifetimeEventNode<N, B, BoundSet<B>>) -> bool {
        return self.s.goal(&n.node);
    }

    fn g_cost(&mut self, n: &LifetimeEventNode<N, B, BoundSet<B>>) -> B {
        return self.s.g_cost(&n.node);
    }

    fn restart(&mut self, msg: String) {
        // reinitializes the boundSet
        self.bound_set.borrow_mut().reset();
        self.s.restart(msg);
    }

    fn handle_new_best(&mut self, n: LifetimeEventNode<N, B, BoundSet<B>>) -> LifetimeEventNode<N, B, BoundSet<B>> {
        LifetimeEventNode {
            node: self.s.handle_new_best(n.node.clone()),
            bound: n.bound,
            lifetime_listener: n.lifetime_listener.clone(),
            expanded: n.expanded,
        }
    }

    fn stop_search(&mut self, _msg: String) {
        self.s.stop_search(_msg);
    }

    fn display_statistics(&self) {
        println!();
        match self.bound_set.borrow().global_bound {
            None => println!("{:>25}{:>15}", "global dual bound", "infeasible"),
            Some(v) => println!("{:>25}{:>15}", "global dual bound", v),
        }
        println!();
        self.s.display_statistics();
    }

    fn export_statistics(&self, json:&mut serde_json::Value) {
        match self.bound_set.borrow().global_bound {
            None => {},
            Some(v) => {
                json["global_dual_bound"] = serde_json::json!(v)
            }
        }
        self.s.export_statistics(json);
    }
}

impl<N, B, Space> TotalNeighborGeneration<LifetimeEventNode<N, B, BoundSet<B>>> for BoundingDecorator<Space, B>
where
    Space: TotalNeighborGeneration<N>+SearchSpace<N,B>,
    B: Ord+Display+Copy+Into<i64>
{
    fn neighbors(&mut self, n: &mut LifetimeEventNode<N, B, BoundSet<B>>) -> Vec<LifetimeEventNode<N, B, BoundSet<B>>> {
        let children = self.s.neighbors(&mut n.node);
        // create node wrappers
        let mut res:Vec<LifetimeEventNode<N, B, BoundSet<B>>> = Vec::new();
        for e in children {
            let bound_e = self.s.bound(&e);
            self.insert_bound(&bound_e);
            res.push(LifetimeEventNode {
                node: e,
                bound: bound_e,
                lifetime_listener: self.bound_set.clone(),
                expanded: false,
            });
        }
        n.expanded = true;
        return res;
    }
}


impl<Space, B> BoundingDecorator<Space, B> where B:Ord+Display+Copy+Into<i64> {
    pub fn unwrap(&self) -> &Space {
        return &self.s;
    }

    pub fn new<N>(s: Space) -> Self
    where B:Ord+Into<i64> {
        Self {s: s, bound_set:Rc::new(RefCell::new(BoundSet::new(Weak::new())))}
    }

    pub fn insert_bound(&mut self, bound:&B) {
        self.bound_set.borrow_mut().insert(bound);
    }

    pub fn bind_logger(self, logger_ref:Weak<MetricLogger>) -> Self {
        if let Some(logger) = logger_ref.upgrade() {
            // adds headers to the logger
            let tmp = logger.register_headers([
                format!("{:<10}","dual"),
            ].to_vec());
            self.bound_set.as_ref().borrow_mut().logging_id_bound = Some(tmp[0]);
        }
        // registers the logger
        self.bound_set.as_ref().borrow_mut().logger = logger_ref.clone();
        return self;
    }

}

impl<N, B, Id, Space> Identifiable<N, Id> for BoundingDecorator<Space, B>
where
    Space: Identifiable<N, Id>,
{
    fn id(&self, n: &N) -> Id {
        return self.s.id(n);
    }
}


impl<N,Space,B> ParetoDominanceSpace<N> for BoundingDecorator<Space, B>
where Space: ParetoDominanceSpace<N>,
{
    fn dominates(&self, a:&N, b:&N) -> bool {
        return self.s.dominates(a,b);
    }
}

impl<N,Space,B> PartialNeighborGeneration<LifetimeEventNode<N, B, BoundSet<B>>> for BoundingDecorator<Space, B>
where
    Space: PartialNeighborGeneration<N>+SearchSpace<N,B>,
    B: Ord+Copy+Into<i64>+Display
{
    fn next_neighbor(&mut self, node: &mut LifetimeEventNode<N, B, BoundSet<B>>) -> Option<LifetimeEventNode<N, B, BoundSet<B>>> {
        match self.s.next_neighbor(&mut node.node) {
            None => { node.expanded = true; return None; }
            Some(c) => {
                let bound = self.s.bound(&c);
                self.insert_bound(&bound);
                return Some(LifetimeEventNode {
                    node: c,
                    bound: bound,
                    lifetime_listener: self.bound_set.clone(),
                    expanded: false,
                });
            }
        }
    }
}