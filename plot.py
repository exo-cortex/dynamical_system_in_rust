#!/usr/bin/env python3

import matplotlib.pyplot as plt
import numpy as np

import glob

file_names = glob.glob("./data/*.txt")
files = [np.loadtxt(fn) for fn in file_names]

for name in file_names:
    print(name)


offs = 0
for (i, file) in enumerate(files):
    if i % 3 == 0:
        offs += 20
    plt.plot(file[:,0], offs + file[:,1], "-", ms=0.5, linewidth=1.125, label="{}".format(file_names[i]))

plt.legend()
plt.show()