#!/usr/bin/python3
"""
executes a command for each instance (typically an optimization program)
for instance:
    ../dogs/scripts/runForInstances.py insts/taillard_flowtime.csv insts/ 'tsp ./target/release/dogs-pfsp -p "results/f_flowtime_alpha/#N.json" -s "results/f_flowtime_alpha/#N.sol" -i "insts/Taillard/tai200_10_5.txt" -t #T f_flowtime -g alpha'
"""

from sys import argv
import csv

if __name__ == "__main__":
    if len(argv) < 3:
        print("USAGE: {} INSTANCE_DATA.csv COMMAND".format(argv[0]))
        print("\t INSTANCE_DATA.csv: path of the instance data file (requires columns: path, time_limit)")
        print("\t COMMAND: COMMAND TO USE")
        print("\t \t #P will be replaced by the instance path")
        print("\t \t #N will be replaced by the instance name")
        print("\t \t #T will be replaced by the time limit")
        exit(1)
    # if correct arguments, read them
    data_filename = argv[1]
    command = argv[3]
    # read csv file and extract instances
    with open(data_filename) as f:
        reader = csv.DictReader(f)
        for row in reader:
            timelimit = row["time_limit"]
            inst_path = row["path"]
            inst_name = inst_path.split("/")[-1]
            c = command.replace("#P", inst_path).replace("#N", inst_name).replace("#T", timelimit)
            print(c)


    