/**
Defines a search space. A search space is a graph structure (directed or not).
It contains nodes. The initial state is the entry point of the search procedure. 
*/
pub trait SearchSpace<N,B> {
    /**
        returns the initial (or root) node
    */
    fn initial(&mut self) -> N;

    /**
        returns the bound value of the node (i.e. f-cost)
     */
    fn bound(&mut self, node: &N) -> B;
 
    /**
        returns true if and only if the node is goal
     */
    fn goal(&mut self, node: &N) -> bool;
 
    /**
        returns the g-cost of the node
        the h-cost can be computed by substracting the f-cost (SearchSpace.bound) by the g-cost
    */
    fn g_cost(&mut self, n: &N) -> B;

    /**
     called when the algorithm finds a new-best-known solution
     */
    fn handle_new_best(&mut self, node: N) -> N { node }



    // RESTARTING INFORMATION

    /**
     called when the algorithm starts
     */
    fn start_search(&mut self, _msg: String) {}

    /**
     called when the algorithm restarts
     */
    fn restart(&mut self, _msg: String) {}

    /**
     called when the algorithm finishes
     */
    fn stop_search(&mut self, _msg: String) {}


    // GETTING SEARCH INFORMATION

    /**
     displays various statistics about the search (nb nodes, etc.)
     */
    fn display_statistics(&self) {}

    /**
     registers information about the search statistics in a json file
     */
    fn json_statistics(&self, _json:&mut serde_json::Value) {}

    /**
     * requests log headers (does nothing if there is no logging decorator within the algorithm)
     */
    fn request_log_header(&self, _res:Vec<String>) {}

    /**
     * requests a logging to appear (does nothing if there is no logging decorator within the algorithm)
     */
    fn request_logging(&self, _res:Vec<String>) {}
}

/**
exports a node to a problem solution Sol.
*/
pub trait ToSolution<N,Sol> {
    /**
     constructs a solution from a goal node
     panics if the node is not a goal
    */
    fn solution(&mut self, node: &mut N) -> Sol;
}

/**
Defines a guidance function
*/
pub trait GuidedSpace<N,G> {
    /**
     * returns the guide value of the node
     */
    fn guide(&mut self, node: &N) -> G;
}


/**
    implements a total neighbor expansion (neighbors)
 */
pub trait TotalNeighborGeneration<N> {
    /**
        returns all neighbors of a given node
     */
    fn neighbors(&mut self, node: &mut N) -> Vec<N>;
}


/**
    implements a partial neighbor expansion (next_neighbor)
 */
pub trait PartialNeighborGeneration<N> {
    /**
        returns the next neighbor if it exists, or None
     */
    fn next_neighbor(&mut self, node: &mut N) -> Option<N>;
}


/**
    Allows to identify a node. Useful to implement g-cost-dominance / bucket-lists
    or tabu-search
 */
pub trait Identifiable<N, Id> {
    /**
        returns the ID of the node. Used to represent a node in which multiple paths may lead
        to the same node (DAG for instance)
     */
    fn id(&self, n: &mut N) -> Id;
}


/**
    Represents a search space where nodes can pareto-dominate some other.
    *i.e.* a node dominantes another if all of its features are better
 */
pub trait ParetoDominanceSpace<N> {
    /**
        returns true if a dominates b
     */
    fn dominates(&self, a:&N, b:&N) -> bool;
}


/**
    Represents a search space which there exists a bound on the distance between the root node and
    any node. This kind of property is frequent in combinatorial branch-and-bounds.
*/
pub trait BoundedDistanceSpace<N> {
    /** maximum distance between the root node and a state */
    fn maximum_root_distance(&self) -> usize;

    /** returns the distance from the root of a given node */
    fn distance_from_root(&self, n:&N) -> usize;

    /** returns the distance from root ratio (distance/distance_max) */
    fn root_distance_ratio(&self, n:&N) -> f64 {
        self.distance_from_root(n) as f64 / self.maximum_root_distance() as f64
    }
}


/** DecisionSpace. Each node can provide a decision
    common usages:
    - tabu search (forbid nodes providing a recent decision already taken: break cycles)
    - search with reinforcement learning (each decision corresponds to a switch between a MDP states)
*/
pub trait DecisionSpace<N,D> {
    /// gets the decision from the node
    fn decision(&self, n:&N) -> Option<D>;
}