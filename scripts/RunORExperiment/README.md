# RunORExeriment

Takes as an input a (kinda) standardized OR experiment format.
Performs the experiments, stores the solutions and the performance statistics
for each solver, then generates various performance analysis.

## Features

 - [X] Read experiment description
 - [X] Read instance list
 - [X] Run the experiments
 - [X] Generate primal bounds table
 - [X] Generate average relative percentage deviation tables
 - [X] Generate Pareto fronts for each instance class
 - [X] add time to best-known
 - [X] add time to optimality proof
 - [X] add which algorithm is statistically better than the others ARPD
 - [X] add latex table for ARPD
 - [X] Handle single-point multiple primal algorithms
 - [X] handle analysis generation only
 - [X] Handle reference for ARPD != BK_ARPD
 - [X] Handle external "primal/time" ARPD
 - [ ] custom Pareto diagrams (subset of solvers)
 - [ ] custom ARPD tables with custom time_limit for curves + inclusion of external ARPD
 - [ ] Handle external "primal/time"
 - [ ] re-run missed experiments with lower number of threads