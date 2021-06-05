#!/usr/bin/python3
"""
takes an instance list (csv format), a file prefix/suffix, and builds a table containing the results
"""

import json
from sys import argv
import csv

def main(data_filename, prefix, suffix):
    # read instance list
    l = []
    with open(data_filename) as f:
        reader = csv.DictReader(f)
        for row in reader:
            e = row["path"].split("/")[-1]
            l.append(e)
    l = list(map(lambda e:prefix+"/"+e+suffix, l))
    res = []
    for e in l:
        with open(e, 'r') as f:
            data = json.load(f)
            res.append(data["stats_pareto"][-1]["v"])
    for e in res:
        print(e)


if __name__ == "__main__":
    if len(argv) < 4:
        print("USAGE: {} INSTANCE_LIST FILE_PREFIX FILE_SUFFIX".format(argv[0]))
    else:
        main(argv[1], argv[2], argv[3])