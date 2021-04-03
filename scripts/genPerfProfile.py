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
    titles = []
    for filename in args[1:-2]:
        with open(filename, 'r') as f:
            data = json.load(f)
            title = data['title']
            titles.append(title)
            x = [ e[x_label] for e in data['points'] ]
            y = [ e[y_label] for e in data['points'] ]
            plt.plot(x, y, drawstyle='steps-post')
            plt.ylabel(y_label)
            plt.xlabel(x_label)
    # plt.xscale("log")
    # plt.yscale("log")
    plt.legend(titles)
    plt.show()



if __name__ == "__main__":
    if len(sys.argv) < 4:
        print("USAGE: {} LIST_OF_PERFPROFILE_FILES x_metric y_metric".format(sys.argv[0]))
        print("\t metric possible values: ['t', 'eval', 'expanded', 'generated', 'goals', 'guided', 'root', 'solutions', 'trashed']")
        exit(1)
    main(sys.argv)
