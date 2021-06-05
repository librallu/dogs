# DOGS (Discrete Optimization Global Search) framework

Implements various search algorithms within a unified paradigm (so far, mostly anytime tree search algorithms).
see [this thesis](https://www.researchgate.net/publication/346063021_Anytime_tree_search_for_combinatorial_optimization) for more information about anytime tree search algorithms.

## implemented components

### tree search algorithms

- [X] Partial Expansion Greedy algorithm
- [X] Beam Search
- [X] Best First Search
- [X] Depth first Search
- [X] Iterative Beam Search
- [X] Limited Discrepancy Search
- [X] Partial Expansion (Iterative) Beam Search

### decorators

- [X] Bounding decorator: measures dual bounds
- [X] LDS decorator: limits the exploration of the tree to the nodes with few discrepancies
- [X] PrefixEquivalence dominance decorator: implements prefix equivalence dominances
- [X] Pruning combinator: prunes nodes that are dominated by the best-known solution
- [X] Statistics combinator: reports various statistics of the search


### TODO

- [ ] replace display_statistics by a function that displays statistics from export_stats (json format)

## examples

Some examples are available for various problems. For some of them, the DOGS implementation is state-of-the-art.

- The sequential ordering problem (SOP) [git repository](https://github.com/librallu/dogs-sop), [reference paper](https://www.researchgate.net/publication/343267812_Tree_search_for_the_Sequential_Ordering_Problem)
- The permutation flowshop (makespan and flowtime minimization) [git repository](https://github.com/librallu/dogs-pfsp), [reference paper](https://www.researchgate.net/publication/344219325_Iterative_beam_search_algorithms_for_the_permutation_flowshop)


## Some helpful tips


### profiling rust applications (linux)

1. install requirements ```sudo apt install -y linux-tools-common linux-tools-generic```
2. install flamegraph via cargo ```cargo install flamegraph```
3. disable the sudo requirement for perf: ```echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid```
4. add the following in the ``Cargo.toml``:
```rust
[profile.release]
debug = true
```
5. ```cargo flamegraph ARGUMENTS```. For instance (SOP): ```cargo flamegraph insts/R.700.1000.15.sop 30```
6. visualize the flamegraph (here by using firefox): ```firefox flamegraph.svg```



### iterating over files (linux)

```bash
for f in `ls DIRNAME/*`; do COMMAND "${f}"; done
```