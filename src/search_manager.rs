use std::cmp::{PartialOrd};
use std::time::{Duration, SystemTime};


/**
 * handles common mechanisms best known solutions in a search algortihm.
 * provides mechanisms to update the best known solution
 */
pub struct SearchManager<N, B> {
    t_start: SystemTime,
    best: Option<N>,
    best_val: Option<B>,
}

impl<N:Clone, B:PartialOrd+Copy> Default for SearchManager<N, B> {
    fn default() -> Self {
        SearchManager {
            t_start: SystemTime::now(),
            best: None,
            best_val: None,
        }
    }
}

impl<N:Clone, B:PartialOrd+Copy> SearchManager<N, B> {

    /**
     * returns the best known solution if it exists
     */
    pub fn best(&self) -> &Option<N> { &self.best }

    /**
     * returns the best known primal value (objective) if it exists
     */
    pub fn best_val(&self) -> &Option<B> { &self.best_val }

    /**
     * returns the elapsed time since the beginning of the search
     */
    pub fn elapsed_time(&self) -> Duration { self.t_start.elapsed().unwrap() }

    /**
     * returns true if current objective is better than the best known solution objective
     */
    pub fn is_better(&self, e: B) -> bool {
        match self.best_val {
            Some(a) => e < a,
            None => true,
        }
    }

    /**
     * updates the best solution and objective if it is dominated
     */
    pub fn update_best(&mut self, s: N, e: B) {
        if self.is_better(e) {
            self.best = Some(s);
            self.best_val = Some(e);
        }
    }

    /**
     * updates another manager to contain the same information
     */
    pub fn give_best(&mut self, other: &mut Self) {
        match self.best_val {
            None => {},
            Some(b) => {
                other.update_best(self.best.as_ref().unwrap().clone(), b);
            }
        };
    }
}
