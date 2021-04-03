#!/usr/bin/python3
import json
from sys import argv

"""
takes an instance list (csv format), a file prefix/suffix, and builds a table containing the results
"""

def main(instances_list, prefix, suffix):
    # read instance list
    l = []
    with open(instances_list, "r") as f:
        names = list(map(lambda e:e.replace("\n","").replace("TA","TA_"), f.readlines()))
    l = list(map(lambda e:prefix+e+suffix, names))
    res = []
    for e in l:
        with open(e, 'r') as f:
            data = json.load(f)
            res.append(data["stats_pareto"][-1]["v"])
    for i in range(len(names)):
        print("{},{}".format(names[i],res[i]))


if __name__ == "__main__":
    if len(argv) < 4:
        print("USAGE: {} INSTANCE_LIST FILE_PREFIX FILE_SUFFIX".format(sys.argv[0]))
    else:
        main(argv[1], argv[2], argv[3])