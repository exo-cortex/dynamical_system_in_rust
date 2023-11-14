#!/usr/bin/env python3

import matplotlib.pyplot as plt
import matplotlib.colors as colors
import numpy as np
import glob
import argparse

parser = argparse.ArgumentParser()
parser.add_argument('-v', '--vars', nargs='*', required=False, type=int, default=[-1])
parser.add_argument('-vs', '--variable-separation', required=False, type=float, default=5)
parser.add_argument('-ns', '--node-separation', required=False, type=float, default=0)
args = parser.parse_args()

file_names = glob.glob("./data/parametric*.txt")
files = [np.loadtxt(fn) for fn in file_names]

# print(files[0][:,0])

variable_names = list(set([fn[27:][0:-4] for fn in file_names]))
print(variable_names)

num_variables = len(variable_names)
num_nodes = len(file_names) // num_variables

print("name(s) of dynamic variable(s): {}".format(variable_names))

node_separation = args.node_separation
variable_separation = args.variable_separation

line_styles = ["-", ":", "-.", "--"]
line_colors = ['#1f77b4', '#ff7f0e', '#2ca02c', '#d62728', '#9467bd', '#8c564b', '#e377c2', '#7f7f7f', '#bcbd22', '#17becf']

# def color(ang):
#     return colors.hsv_to_rgb([ang, 1,0.8])

for (i, file) in enumerate(files):
    color = line_colors[i % len(line_colors)]
    offset = i * node_separation
    plt.plot(file[:,0], offset + file[:,1], "-", ms=0.5, linewidth=0.9, label="{}".format(variable_names[i % num_variables]), color=color)

plt.legend()
plt.show()