pub mod searchalgorithm;
pub mod searchmanager;
pub mod searchspace;
pub mod metriclogger;

pub mod treesearch;
pub mod genetic;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
