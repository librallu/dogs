#!/usr/bin/python3

"""
takes a list of perfprofiles and x,y values to display
"""


import sys
import json
import matplotlib.pyplot as plt

def main(args):
    x_label = args[-2]
    y_label = args[-1]
    algo_names = []
    title = ""
    for filename in args[1:-2]:
        with open(filename, 'r') as f:
            data = json.load(f)
            # title = data['title']
            # titles.append(title)
            if "inst" in data:
                title = data["inst"]
            if "algo" in data:
                algo_names.append(data["algo"])
            else:
                algo_names.append("?")
            x = [ e[x_label] for e in data['stats_pareto'] ]
            y = [ e[y_label] for e in data['stats_pareto'] ]
            plt.plot(x, y, drawstyle='steps-post')
            plt.ylabel(y_label)
            plt.xlabel(x_label)
    # plt.xscale("log")
    # plt.yscale("log")
    plt.legend(algo_names)
    plt.title(title)
    plt.show()



if __name__ == "__main__":
    if len(sys.argv) < 4:
        print("USAGE: {} LIST_OF_PERFPROFILE_FILES x_metric y_metric".format(sys.argv[0]))
        print("\t metric possible values: ['t', 'eval', 'expanded', 'generated', 'goals', 'guided', 'root', 'solutions', 'trashed']")
        exit(1)
    main(sys.argv)
