use std::fmt::{Display, Debug};
use std::time::{SystemTime};
use std::io::{Write};
use std::fs::File;
use std::rc::{Weak};

use serde::Serialize;
use serde_json::json;

extern crate human_format;

use crate::metriclogger::{Metric, MetricLogger};
use crate::searchspace::{SearchSpace, GuidedSpace, PrefixEquivalenceTree, SearchTree, TotalChildrenExpansion, PartialChildrenExpansion, ParetoDominanceSpace};


/// search statistics data (at a given time)
#[derive(Clone, Debug)]
pub struct PerfProfilePoint {
    expanded: u64,
    generated: u64,
    root: u64,
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
            root: 0,
            eval: 0,
            goals: 0,
            trashed: 0,
            solutions: 0,
            guide: 0,
        }
    }
}

pub struct StatTsDecorator<Tree, B:Serialize> {
    s: Tree,
    stats: PerfProfilePoint,
    t_start: SystemTime,
    nb_sols: u64,
    perfprofile: Vec<PerfProfileEntry<B>>,
    logger: Weak<MetricLogger>,
    logging_id_nbnodes: Option<usize>,
    logging_id_obj: Option<usize>,
}

impl<N,G,Tree,B> GuidedSpace<N,G> for StatTsDecorator<Tree,B>
where 
    Tree: GuidedSpace<N,G>,
    B: Serialize
{
    fn guide(&mut self, node: &N) -> G {
        self.stats.guide += 1;
        return self.s.guide(node);
    }
}

impl<N,Sol,Tree,B> SearchSpace<N,Sol> for StatTsDecorator<Tree,B>
where 
    Tree: SearchSpace<N,Sol>+SearchTree<N,B>,
    B: Clone+Serialize+Into<i64>
{
    fn solution(&mut self, node: &N) -> Sol {
        self.stats.solutions += 1;
        return self.s.solution(node);
    }

    fn restart(&mut self, msg: String) {
        self.s.restart(msg);
    }

    fn handle_new_best(&mut self, n: &N) {
        self.nb_sols += 1;
        let obj = self.s.bound(n);
        self.perfprofile.push(PerfProfileEntry {
            p: self.stats.clone(),
            t: self.t_start.elapsed().unwrap().as_secs_f32(),
            v: Some(obj.clone())
        });
        // updates logger and display statistics
        if let Some(logger) = self.logger.upgrade() {
            if let Some(id) = self.logging_id_obj {
                logger.update_metric(id, Metric::Int(obj.into()));
                logger.request_logging();
            }
        }
        self.s.handle_new_best(n);
    }

    /// adds factice point when the search stops
    fn stop_search(&mut self, _msg: String) {
        self.s.stop_search(_msg);
        self.perfprofile.push(PerfProfileEntry {
            p: self.stats.clone(),
            t: self.t_start.elapsed().unwrap().as_secs_f32(),
            v: match self.perfprofile.len() {
                0 => None,
                n => self.perfprofile[n-1].v.clone()
            }
        });
    }

    fn export_statistics(&self, json:&mut serde_json::Value) {
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
        self.s.export_statistics(json);
    }

    fn display_statistics(&self) {
        self.s.display_statistics();
        let time = self.t_start.elapsed().unwrap().as_secs_f32();
        let format = |e| human_format::Formatter::new().with_decimals(1).format(e);
        println!("");
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


impl<'a, N, B, Tree> SearchTree<N, B> for StatTsDecorator<Tree, B>
where
    B: Display+Debug+Copy+Serialize,
    Tree: SearchTree<N, B>+TotalChildrenExpansion<N>,
{
    fn root(&mut self) -> N {
        self.stats.root += 1;
        return self.s.root();
    }

    fn bound(&mut self, node: &N) -> B {
        self.stats.eval += 1;
        return self.s.bound(node);
    }

    fn goal(&mut self, node: &N) -> bool {
        self.stats.goals += 1;
        return self.s.goal(node);
    }
}


impl<N, Tree, B> TotalChildrenExpansion<N> for StatTsDecorator<Tree,B>
where 
    Tree: TotalChildrenExpansion<N>,
    B: Serialize,
{

    fn children(&mut self, node: &mut N) -> Vec<N> {
        let res = self.s.children(node);
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
        return res;
    }
}

impl<N, Tree, B> PartialChildrenExpansion<N> for StatTsDecorator<Tree,B>
where 
    Tree: PartialChildrenExpansion<N>,
    B: Serialize,
{

    fn get_next_child(&mut self, node: &mut N) -> Option<N> {
        let res = self.s.get_next_child(node);
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
        return res;
    }
}


impl<N, B, PE, Tree> PrefixEquivalenceTree<N, B, PE> for StatTsDecorator<Tree, B>
where
    Tree: PrefixEquivalenceTree<N, B, PE>,
    B: Serialize,
{
    fn get_pe(&self, n: &N) -> PE {
        return self.s.get_pe(n);
    }

    fn prefix_bound(&self, n: &N) -> B {
        return self.s.prefix_bound(n);
    }
}

impl<Tree, B:Serialize+Copy+Display> StatTsDecorator<Tree, B> {
    pub fn new(s: Tree) -> Self {
        let res = Self {
            s: s,
            stats: PerfProfilePoint::new(),
            t_start: SystemTime::now(),
            nb_sols: 0,
            perfprofile: Vec::new(),
            logger: Weak::new(),
            logging_id_nbnodes: None,
            logging_id_obj: None,
        };
        return res;
    }

    pub fn unwrap(&self) -> &Tree {
        return &self.s;
    }

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
        return self;
    }

    pub fn export_performance_profile(&self, filename:&str, title:&str) {
        let mut file = match File::create(filename) {
            Err(why) => panic!("couldn't create {}: {}", filename, why),
            Ok(file) => file
        };
        let mut res = json!({});
        res["title"] = json!(title);
        let mut points:Vec<serde_json::Value> = vec![];
        for e in &self.perfprofile {
            let mut tmp = json!({
                "expanded": e.p.expanded,
                "generated": e.p.generated,
                "root": e.p.root,
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
        res["points"] = json!(points);
        match file.write(serde_json::to_string(&res)
        .unwrap().as_bytes()) {
            Err(why) => panic!("couldn't write: {}",why),
            Ok(_) => {}
        };
    }

}


impl<N,Tree,B> ParetoDominanceSpace<N> for StatTsDecorator<Tree, B>
where 
    Tree: ParetoDominanceSpace<N>,
    B: Serialize,
{
    fn dominates(&self, a:&N, b:&N) -> bool {
        return self.s.dominates(a,b);
    }
}