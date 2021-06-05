// directories
pub mod tree_search;
pub mod genetic;
pub mod local_search;
pub mod data_structures;

// files
pub mod search_algorithm;
pub mod search_manager;
pub mod search_space;
pub mod metric_logger;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
