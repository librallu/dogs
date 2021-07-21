use std::fmt::{Display, Debug};
use std::time::SystemTime;
use std::rc::Weak;

use serde::Serialize;
use serde_json::json;

use crate::metric_logger::{Metric, MetricLogger};
use crate::search_space::{SearchSpace, GuidedSpace, Identifiable, TotalNeighborGeneration, PartialNeighborGeneration, ParetoDominanceSpace, ToSolution};
use crate::search_decorator::SearchSpaceDecorator;

/// search statistics data (at a given time)
#[derive(Clone, Debug)]
pub struct PerfProfilePoint {
    expanded: u64,
    generated: u64,
    initial: u64,
    eval: u64,
    goals: u64,
    trashed: u64,
    solutions: u64,
    guide: u64,
}

/// performance profile entry (PerfProfilePoint + time + solution value)
#[derive(Debug)]
pub struct PerfProfileEntry<B> {
    p: PerfProfilePoint,
    t: f32,
    v: Option<B>
}

/// Statistics tree search Decorator
impl PerfProfilePoint {
    fn new() -> PerfProfilePoint {
        PerfProfilePoint {
            expanded: 0,
            generated: 0,
            initial: 0,
            eval: 0,
            goals: 0,
            trashed: 0,
            solutions: 0,
            guide: 0,
        }
    }
}

/** stats decorator. Stores statistics data-structures and reference to the logger. */
#[derive(Debug)]
pub struct StatTsDecorator<Space, B> {
    s: Space,
    stats: PerfProfilePoint,
    t_start: SystemTime,
    nb_sols: u64,
    perfprofile: Vec<PerfProfileEntry<B>>,
    logger: Weak<MetricLogger>,
    logging_id_nbnodes: Option<usize>,
    logging_id_obj: Option<usize>,
}

impl<N,G,Space,B> GuidedSpace<N,G> for StatTsDecorator<Space,B>
where 
    Space: GuidedSpace<N,G>,
    B: Serialize
{
    fn guide(&mut self, node: &N) -> G {
        self.stats.guide += 1;
        self.s.guide(node)
    }
}


impl<N,Sol,Space,B> ToSolution<N,Sol> for StatTsDecorator<Space, B>
where Space:ToSolution<N,Sol>, B:serde::Serialize {
    fn solution(&mut self, node: &mut N) -> Sol {
        self.s.solution(node)
    }
}


impl<N,Space,B> SearchSpace<N,B> for StatTsDecorator<Space,B>
where 
    Space: SearchSpace<N,B>,
    B: Clone+Serialize+Into<i64>+PartialOrd+Copy+std::fmt::Display
{

    fn initial(&mut self) -> N {
        self.stats.initial += 1;
        self.s.initial()
    }

    fn bound(&mut self, node: &N) -> B {
        self.stats.eval += 1;
        self.s.bound(node)
    }

    /**
     * TODO maybe have a g_cost stat?
     */
    fn g_cost(&mut self, node: &N) -> B {
        self.stats.eval += 1;
        self.s.g_cost(node)
    }

    fn goal(&mut self, node: &N) -> bool {
        self.stats.goals += 1;
        self.s.goal(node)
    }

    fn restart(&mut self, msg: String) {
        self.s.restart(msg);
    }

    fn handle_new_best(&mut self, n: N) -> N {
        // update before handle best
        let obj1 = self.s.bound(&n);
        self.nb_sols += 1;
        self.perfprofile.push(PerfProfileEntry {
            p: self.stats.clone(),
            t: self.t_start.elapsed().unwrap().as_secs_f32(),
            v: Some(obj1)
        });
        // updates logger and display statistics
        if let Some(logger) = self.logger.upgrade() {
            if let Some(id) = self.logging_id_obj {
                logger.update_metric(id, Metric::Int(obj1.into()));
                logger.request_logging();
            }
        }
        // call the handle new best (possibly improve the solution)
        let n2 = self.s.handle_new_best(n);
        self.nb_sols += 1;
        let obj2 = self.s.bound(&n2);
        self.perfprofile.push(PerfProfileEntry {
            p: self.stats.clone(),
            t: self.t_start.elapsed().unwrap().as_secs_f32(),
            v: Some(obj2)
        });
        // updates logger and display statistics
        if obj2 < obj1 {
            if let Some(logger) = self.logger.upgrade() {
                if let Some(id) = self.logging_id_obj {
                    logger.update_metric(id, Metric::Int(obj2.into()));
                    logger.request_logging();
                }
            }
        }
        n2
    }

    /// adds factice point when the search stops
    fn stop_search(&mut self, _msg: String) {
        self.s.stop_search(_msg);
        self.perfprofile.push(PerfProfileEntry {
            p: self.stats.clone(),
            t: self.t_start.elapsed().unwrap().as_secs_f32(),
            v: match self.perfprofile.len() {
                0 => None,
                n => self.perfprofile[n-1].v
            }
        });
    }

    fn json_statistics(&self, json:&mut serde_json::Value) {
        let time = self.t_start.elapsed().unwrap().as_secs_f32();
        json["nb_generated"] = serde_json::json!(self.stats.generated);
        json["nb_expanded"] = serde_json::json!(self.stats.expanded);
        json["nb_trashed"] = serde_json::json!(self.stats.trashed);
        if self.stats.expanded > 0 {
            json["avg_branching_factor"] = serde_json::json!((self.stats.generated as f64) / (self.stats.expanded as f64));
        }
        json["nb_eval"] = serde_json::json!(self.stats.eval);
        json["nb_guide"] = serde_json::json!(self.stats.guide);
        json["time_searched"] = serde_json::json!(time);
        json["nodes_generated_per_sec"] = serde_json::json!((self.stats.generated as f32) / time);
        json["nb_sols_created"] = serde_json::json!(self.stats.solutions);
        json["primal_pareto_diagram"] = self.get_pareto_diagram();
        self.s.json_statistics(json);
    }

    fn display_statistics(&self) {
        self.s.display_statistics();
        let time = self.t_start.elapsed().unwrap().as_secs_f32();
        let format = |e| human_format::Formatter::new().with_decimals(1).format(e);
        println!();
        println!(
            "{:>25}{:>15}",
            "nb generated",
            format(self.stats.generated as f64)
        );
        println!(
            "{:>25}{:>15}",
            "nb expanded",
            format(self.stats.expanded as f64)
        );
        println!(
            "{:>25}{:>15}",
            "nb trashed",
            format(self.stats.trashed as f64)
        );
        if self.stats.expanded > 0 {
            println!(
                "{:>25}{:>15.3}",
                "avg branching factor",
                format((self.stats.generated as f64) / (self.stats.expanded as f64))
            );
        }
        println!("{:>25}{:>15}", "nb eval", format(self.stats.eval as f64));
        println!("{:>25}{:>15}", "nb guide", format(self.stats.guide as f64));
        println!("{:>25}{:>15.3}", "time searched (s)", time);
        println!(
            "{:>25}{:>15}",
            "generated nodes / s",
            format(((self.stats.generated as f32) / time) as f64)
        );
        println!(
            "{:>25}{:>15}",
            "solutions created",
            format(self.stats.solutions as f64)
        );
    }
}


impl<N, Space, B> TotalNeighborGeneration<N> for StatTsDecorator<Space,B>
where 
    Space: TotalNeighborGeneration<N>,
    B: Serialize,
{

    fn neighbors(&mut self, node: &mut N) -> Vec<N> {
        let res = self.s.neighbors(node);
        self.stats.expanded += 1;
        self.stats.generated += (res.len()) as u64;
        if res.is_empty() {
            self.stats.trashed += 1;
        }
        // updates logger and display statistics
        if let Some(logger) = self.logger.upgrade() {
            if let Some(id) = self.logging_id_nbnodes {
                logger.update_metric(
                    id,
                    Metric::LargeNumber(self.stats.generated as f64)
                );
            }
        }
        res
    }
}

impl<N, Space, B> PartialNeighborGeneration<N> for StatTsDecorator<Space,B>
where 
    Space: PartialNeighborGeneration<N>,
    B: Serialize,
{

    fn next_neighbor(&mut self, node: &mut N) -> Option<N> {
        let res = self.s.next_neighbor(node);
        match &res {
            None => { self.stats.expanded += 1; }
            Some(_) => { self.stats.generated += 1; }
        }
        if let Some(logger) = self.logger.upgrade() {
            if let Some(id) = self.logging_id_nbnodes {
                logger.update_metric(
                    id, Metric::LargeNumber(self.stats.generated as f64)
                );
            }
        }
        res
    }
}


impl<N, B, Id, Space> Identifiable<N, Id> for StatTsDecorator<Space, B>
where
    Space: Identifiable<N, Id>,
    B: Serialize,
{
    fn id(&self, n: &mut N) -> Id { self.s.id(n) }
}

impl<Space, B> SearchSpaceDecorator<Space> for StatTsDecorator<Space, B> {
    fn unwrap(&self) -> &Space { &self.s }
}

impl<Space, B:Serialize+Copy+Display> StatTsDecorator<Space, B> {
    /** builds the decorator around a search space */
    pub fn new(s: Space) -> Self {
        Self {
            s,
            stats: PerfProfilePoint::new(),
            t_start: SystemTime::now(),
            nb_sols: 0,
            perfprofile: Vec::new(),
            logger: Weak::new(),
            logging_id_nbnodes: None,
            logging_id_obj: None,
        }
    }

    /** binds to a logger (to display statistics in the console) */
    pub fn bind_logger(mut self, logger_ref:Weak<MetricLogger>) -> Self {
        if let Some(logger) = logger_ref.upgrade() {
            // adds headers to the logger
            let tmp = logger.register_headers([
                format!("{:<15}","nb nodes"),
                format!("{:<15}","objective"),
            ].to_vec());
            self.logging_id_nbnodes = Some(tmp[0]);
            self.logging_id_obj = Some(tmp[1]);
        }
        // registers the logger
        self.logger = logger_ref;
        self
    }

    /** generates pareto diagram to visualize the search statistics */
    pub fn get_pareto_diagram(&self) -> serde_json::Value {
        let mut points:Vec<serde_json::Value> = vec![];
        for e in &self.perfprofile {
            let mut tmp = json!({
                "expanded": e.p.expanded,
                "generated": e.p.generated,
                "initial": e.p.initial,
                "eval": e.p.eval,
                "goals": e.p.goals,
                "trashed": e.p.trashed,
                "solutions": e.p.solutions,
                "guide": e.p.guide,
                "t": e.t
            });
            match e.v {
                None => {},
                Some(v) => tmp["v"] = json!(v)
            };
            points.push(tmp);
        }
        json!(points)
    }

}


impl<N,Space,B> ParetoDominanceSpace<N> for StatTsDecorator<Space, B>
where 
    Space: ParetoDominanceSpace<N>,
    B: Serialize,
{
    fn dominates(&self, a:&N, b:&N) -> bool { self.s.dominates(a,b) }
}