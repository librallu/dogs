pub trait SearchSpace<N,Sol> {
    /**
     * constructs a solution from a goal node
     */
    fn solution(&mut self, node: &N) -> Sol;

    /**
     * called when the algorithm finds a new-best-known solution
     */
    fn handle_new_best(&mut self, _node: &N) {}

        // RESTARTING INFORMATION
    /**
     * called when the algorithm starts
     */
    fn start_search(&mut self, _msg: String) {}

    /**
     * called when the algorithm restarts
     */
    fn restart(&mut self, _msg: String) {}

    /**
     * called when the algorithm finishes
     */
    fn stop_search(&mut self, _msg: String) {}


    // GETTING SEARCH INFORMATION

    /**
     * displays various statistics about the search (nb nodes, etc.)
     */
    fn display_statistics(&self) {}

    /**
     * registers information about the search statistics in a json file
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


pub trait GuidedSpace<N,G> {
    /**
     * returns the guide value of the node
     */
    fn guide(&mut self, node: &N) -> G;
}


pub trait SearchTree<N, B> {

    /**
     * returns the root node
     */
    fn root(&mut self) -> N;

    /**
     * returns the bound value of the node
     */
    fn bound(&mut self, node: &N) -> B;

    /**
     * returns true if and only if the node is goal
     */
    fn goal(&mut self, node: &N) -> bool;

}


/**
 * implements a total children expansion (children)
 */
pub trait TotalChildrenExpansion<N> {
    /**
     * returns all children of a given node
     */
    fn children(&mut self, node: &mut N) -> Vec<N>;
}

/**
 * implements a partial children expansion (get_next_child)
 */
pub trait PartialChildrenExpansion<N> {
    /**
     * returns the next child if it exists, or no children
     */
    fn get_next_child(&mut self, node: &mut N) -> Option<N>;
}

/**
 * Represents a prefix equivalence tree.
 */
pub trait PrefixEquivalenceTree<N, B, PE> {
    /**
     * returns a prefix equivalence information
     */
    fn get_pe(&self, n: &N) -> PE;

    /**
     * returns the cost of the prefix
     */
    fn prefix_bound(&self, n: &N) -> B;
}

/**
 * Represents a search space where nodes can pareto-dominate some other.
 */
pub trait ParetoDominanceSpace<N> {
    /**
     * returns true if a dominates b
     */
    fn dominates(&self, a:&N, b:&N) -> bool;
}