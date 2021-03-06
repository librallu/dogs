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
     TODO: use export_statistics to get the statistics in the JSON format, and display them
     */
    fn display_statistics(&self) {}

    /**
     registers information about the search statistics in a json file
     */
    fn export_statistics(&self, _json:&mut serde_json::Value) {}

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
    Allows to identify a node. Useful to implement prefix-equivalences / bucket-lists
    or tabu-search
 */
pub trait Identifiable<N, Id> {
    /**
        returns the ID of the node
     */
    fn id(&self, n: &N) -> Id;
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