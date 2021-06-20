- [ ] for a given set of results, compute if one is statistically better than the others (produce a CSV)
    - arguments: 
- [ ] produce a ARPD/ARPT pareto diagram 

# DOGS scripts

Includes multiple scripts useful to perform experiments and extract results from them.



## runForInstances.py

Reads a CSV file that contains a list of instances, and executes a command for each of them.



## extractTableFromPerfs.py

Given a CSV file that contains a list of instances, and a prefix (resp. suffix) strings. The script
reads each result file and produces a CSV with the results (the best solution found for all instances).



## genParetoDiagram.py

Given a list of performance stats, and x/y metrics, display a performance profile (solution quality
over time).



## genArpd.py

Given a reference CSV file and a results file, display an ARPD Pareto diagram.