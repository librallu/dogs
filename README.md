# DOGS (Discrete Optimization Global Search) framework

Implements various search algorithms within a unified paradigm (so far, mostly anytime tree search algorithms).
See [this thesis](https://www.researchgate.net/publication/346063021_Anytime_tree_search_for_combinatorial_optimization) for more information about anytime tree search algorithms.

## Implemented components

### Tree search algorithms

- [X] Greedy algorithm
- [X] Partial Expansion Greedy algorithm
- [X] Beam Search
- [X] Best First Search
- [X] Depth first Search
- [X] Iterative Beam Search
- [X] Limited Discrepancy Search
- [X] Partial Expansion (Iterative) Beam Search


### Combinators

- [X] Bounding combinator: measures dual bounds
- [X] LDS combinator: limits the exploration of the tree to the nodes with few discrepancies
- [X] G-cost dominance combinator: implements g-cost dominance
- [X] Pruning combinator: prunes nodes that are dominated by the best-known solution
- [X] Statistics combinator: reports various statistics of the search
- [X] Tabu combinator: forbids decisions taken before in the search

### Roadmap: What's next?

- [ ] Reactive tabu management
- [ ] Possible bug in "is_optimal" if the time limit is exceeded before the search makes some
      heuristic fathoming. In this case, the algorithm will report "optimal" while it is not.
- [ ] Add Decorator trait and base implementation for unwrap()
- [ ] improve LazyComputable usage (trait?)


## examples

Some examples are available for various problems. For some of them, the DOGS implementation is state-of-the-art.

- The sequential ordering problem (SOP) [git repository](https://github.com/librallu/dogs-sop), [reference paper](https://www.researchgate.net/publication/343267812_Tree_search_for_the_Sequential_Ordering_Problem)
- The permutation flowshop (makespan and flowtime minimization) [git repository](https://github.com/librallu/dogs-pfsp), [reference paper](https://www.researchgate.net/publication/344219325_Iterative_beam_search_algorithms_for_the_permutation_flowshop)


## Some general helpful tips

### Install rust

See [rust getting started page](https://www.rust-lang.org/learn/get-started).


### Flamegraph profiling (Linux)

1. Install requirements ```sudo apt install -y linux-tools-common linux-tools-generic```
2. Install flamegraph via cargo ```cargo install flamegraph```
3. Disable the sudo requirement for perf: ```echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid```. Possibly, `sudo sh -c 'echo kernel.perf_event_paranoid=1 > /etc/sysctl.d/local.conf'` may allow you to do not use the previous command in every terminal.
4. Add the following in the ``Cargo.toml``:
```rust
[profile.release]
debug = true
```
5. ```cargo flamegraph ARGUMENTS```. For instance (SOP): ```cargo flamegraph insts/R.700.1000.15.sop 30```
6. Visualize the flamegraph (here by using Firefox): ```firefox flamegraph.svg```.



### Heap profiling (Linux)

We recommend using use [heaptrack](https://github.com/KDE/heaptrack).

1. Call `heaptrack PROG`
2. Analyze data `heaptrack_gui DATA.gz`


### Cache performance

We recommend using Valgrind

1. `valgrind --tool=callgrind --dump-instr=yes --collect-jumps=yes --simulate-cache=yes [PROGRAM] [ARGS]`
