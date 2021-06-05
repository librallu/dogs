#!/usr/bin/python3
"""
computes the ARPD table for an instance list (requires a given field)
COL_NAMES being a string that represents columns separated by commas.
"""
import csv
from sys import argv
from scipy.stats import wilcoxon

if __name__ == "__main__":
    if len(argv) < 4:
        print('USAGE: {} REF_FILE.CSV \t RESULT_FILE.CSV \t COL_NAMES'.format(argv[0]))
        exit(1)
    ref_filename = argv[1]
    res_filename = argv[2]
    colnames = (argv[3]).split(",")
    # read instance file
    insts = {}
    classes = {}
    with open(ref_filename) as f:
        reader = csv.DictReader(f)
        for row in reader:
            name = row["name"]
            bk = float(row["bk_primal"])
            c = row["class_name"]
            insts[name] = bk
            if c in classes:
                classes[c].append(name)
            else:
                classes[c] = [name]
    # print(insts)
    # print(classes)
    # COMPUTE RPDs
    rpds = {}
    for colname in colnames:
        rpds[colname] = {}
        with open(res_filename) as f:
            reader = csv.DictReader(f)
            for row in reader:
                name = row["name"]
                v = float(row[colname])
                rpds[colname][name] = (v-insts[name])/insts[name]*100
    # print(rpds)
    # COMPUTE ARPDs
    arpds = {}
    for colname in colnames:
        arpds[colname] = {}
        for c in classes:
            l = []
            for k in classes[c]:
                l.append(rpds[colname][k])
            arpds[colname][c] = sum(l)/len(l)
    # print(arpds)
    # COMPUTE WILCOXON SIGNED TEST
    # for each value, tests if it is statistically better than the others
        #     _,p = wilcoxon([res2[i]-res1[i] for i in range(10)])
        # res['p'] = p
    # DISPLAY TABLE
    print("class,{}".format(",".join(colnames)))
    for c in classes:
        print("{},{}".format(c,",".join([str(arpds[colname][c]) for colname in colnames])))

