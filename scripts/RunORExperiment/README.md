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
 - [X] latex table for ARPD with Wilcoxon signed-rank test
 - [X] Handle single-point multiple primal algorithms
 - [X] handle analysis generation only
 - [X] Handle reference for ARPD != BK_ARPD
 - [X] Handle external "primal/time" ARPD
 - [X] custom Pareto diagrams (subset of solvers)
 - [X] best-known stats (number improved, number reached, number proved to optimality)
 - [X] add custom data in the best_known table
 - [X] generate latex file for best known solutions
 - [X] add spaces in the latex table outputs for easy copy & paste
 - [X] custom ARPD tables with custom time_limit for curves + inclusion of external ARPD
 - [X] re-run missed experiments with lower number of threads
 - [ ] add multiple_y types to the best known table
 - [ ] if Cartesian product over parameters, perform analysis similar to the ROADEF18 paper