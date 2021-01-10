use crate::searchmanager::SearchManager;
use rand::prelude::*;

/**
 * Applies a crossover (generates an individual from 2 parents)
 */
pub trait GeneticIndividual:Clone {
    fn is_feasible(&self) -> bool;
    fn crossover_ox(p1:&Self, p2:&Self) -> Self;
}

pub trait GeneticPopulation<I:GeneticIndividual> {
    /**
     * Applies a binary tournament to extract one individual
     */
    fn get_from_binary_tournament(&self) -> I;

    /**
     * Adds an individual to the population
     */
    fn add_individual(&mut self, individual:&I, b:bool) -> bool;
    
    /**
     * adds penalties if too many iterations ran without improving
     */
    fn manage_penalties(&mut self);

    /**
     * prints current state of the search
     */
    fn print_state(&self, nbiter:usize, nbiter_non_improving:usize);
}


pub trait LocalSearch<I> {
    fn run(&mut self, individual:&mut I, penalty_capacity:f64, penalty_duration:f64) -> I;
}


pub struct Params {
    penaltycapacity: f64,
    penaltyduration: f64,
}

pub struct GeneticSimple<I,O,P,LS> {
    pub manager: SearchManager<I,O>,
    population: P,
    localsearch: LS,
    params: Params
}

impl<'a, I:GeneticIndividual, O:PartialOrd+Copy, P:GeneticPopulation<I>, LS:LocalSearch<I>> GeneticSimple<I,O,P,LS> {

    pub fn run(&mut self, stopping_criterion: impl Fn(&SearchManager<I,O>, &usize)->bool) {
        let mut nbiter_non_improving:usize = 0;
        let mut nbiter:usize = 0;
        while stopping_criterion(&self.manager, &nbiter_non_improving) {
            // selection and crossover
             let mut offspring:I = I::crossover_ox(
                &self.population.get_from_binary_tournament(),
                &self.population.get_from_binary_tournament()
            );
            // local search
            self.localsearch.run(&mut offspring, self.params.penaltycapacity, self.params.penaltyduration);
            let mut is_new_best:bool = self.population.add_individual(&offspring,true);
            if offspring.is_feasible() && random::<bool>() {
                // repair half of the infeasible solutions
                self.localsearch.run(
                    &mut offspring,
                    self.params.penaltycapacity*10.,
                    self.params.penaltyduration*10.
                );
                if offspring.is_feasible() {
                    is_new_best = self.population.add_individual(&offspring,is_new_best) || is_new_best;
                }
            }
            // updating count of nb iterations that are not improving
            if is_new_best {
                nbiter_non_improving = 0;
            } else {
                nbiter_non_improving += 1;
            }
            // diversification mechanism
            if nbiter % 100 == 0 { self.population.manage_penalties(); }
            // display information
            if nbiter % 500 == 0 { self.population.print_state(nbiter, nbiter_non_improving); }
            nbiter += 1;
        }
    }


}



#[cfg(test)]
mod tests {
    use rand::random;

    #[derive(Clone)]
    pub struct BinaryWordIndividual {
        pub w:Vec<bool>
    }
    
    impl super::GeneticIndividual for BinaryWordIndividual {
        fn is_feasible(&self) -> bool {
            return true;
        }
    
        fn crossover_ox(p1:&Self, p2:&Self) -> Self {
            let mut res = p1.clone();
            for (i,e) in p2.w.iter().enumerate() {
                if random::<f64>() < 0.1 {
                    res.w[i] = *e;
                }
            }
            return res;
        }
    }
    
    pub struct BinaryWordPopulation {
        pop:Vec<BinaryWordIndividual>
    }

    impl super::GeneticPopulation<BinaryWordIndividual> for BinaryWordPopulation {
        fn get_from_binary_tournament(&self) -> BinaryWordIndividual {
            // TODO
        }
    
        fn add_individual(&mut self, individual:&BinaryWordIndividual, b:bool) -> bool {
            self.pop.push(individual.clone());
            // TODO tell if it is the best among the others
        }
        
        fn manage_penalties(&mut self) {
            // dummy function (do nothing)
        }
    
        fn print_state(&self, nbiter:usize, nbiter_non_improving:usize) {
            println!("{}\t{}",nbiter, nbiter_non_improving);
        }
    }

    #[test]
    fn test_binary_word() {

    }
}
