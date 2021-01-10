# DOGS (Discrete Optimization Global Search) framework

Implements various search algorithms within a unified paradigm (so far, mostly anytime tree search algorithms).
see [this thesis](https://www.researchgate.net/publication/346063021_Anytime_tree_search_for_combinatorial_optimization) for more information about anytime tree search algorithms.

## implemented components

### tree search algorithms

- [X] Beam Search
- [X] Best First Search
- [X] DFS
- [X] Iterative Beam Search
- [X] LDS

### combinators

- [X] Bounding combinator
- [X] LDS combinator
- [X] PrefixEquivalence dominance combinator
- [X] Pruning combinator
- [X] Statistics combinator


## examples

Some examples are available for various problems. For some of them, the DOGS implementation is state-of-the-art.

- The sequential ordering problem (SOP) TODO
- The permutation flowshop (makespan and flowtime minimization) TODO


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
