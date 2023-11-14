#!/usr/bin/env python3

import matplotlib.pyplot as plt
import matplotlib.colors as colors
import numpy as np
import glob
import argparse

parser = argparse.ArgumentParser()
parser.add_argument('-v', '--vars', nargs='*', required=False, type=int, default=[-1], help="list of variables to plot")
parser.add_argument('-vs', '--variable-separation', required=False, type=float, default=0, help="vertical offset between variables")
parser.add_argument('-ns', '--node-separation', required=False, type=float, default=0, help="vertical offset between nodes")
args = parser.parse_args()

file_names = glob.glob("./data/*.txt")
files = [np.loadtxt(fn) for fn in file_names]

variable_names = list(set([fn[21:][0:-4] for fn in file_names]))

num_variables = len(variable_names)
num_nodes = len(file_names) // num_variables

print("name(s) of dynamic variable(s): {}".format(variable_names))

node_separation = args.node_separation
variable_separation = args.variable_separation

line_styles = ["-", ":", "--", "-."]
line_colors = ['#1f77b4', '#ff7f0e', '#2ca02c', '#d62728', '#9467bd', '#8c564b', '#e377c2', '#7f7f7f', '#bcbd22', '#17becf']

def color(ang):
    return colors.hsv_to_rgb([ang, 1,0.8])

for node_index in range(num_nodes):
    color = line_colors[node_index % len(line_colors)]
    for (i, variable) in enumerate(variable_names):
        if -1 in args.vars or i in args.vars:
            file = files[node_index * num_variables + i]
            offset = node_index * node_separation + i * variable_separation
            plt.plot(file[:,0], offset + file[:,1], line_styles[i], ms=0.5, linewidth=1.125, label="{}_{}".format(variable_names[i], node_index), color=color)

plt.legend()
plt.show()