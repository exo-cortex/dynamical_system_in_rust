#!/usr/bin/env python3

import matplotlib.pyplot as plt
import numpy as np

import glob

# file = np.loadtxt("test.txt")

# for i in range( (len(file[0])-1) // 2):
#     plt.plot(file[:,0], i * 2.0 + file[:,i*2+1], "-", ms=1.5, linewidth=1.25, label="e")
#     plt.plot(file[:,0], i * 2.0 + file[:,i*2+2] * 5.0, "-", ms=1.5, linewidth=1.25, label="n")

file_names = glob.glob("./data/*.txt")
files = [np.loadtxt(fn) for fn in file_names]
# print(files)

for (i, file) in enumerate(files):
    plt.plot(file[:,0], file[:,1], "-", ms=0.5, linewidth=1.125, label="{}".format(file_names[i]))



# shift = 15 * 100//64

# for i in range( (len(file[0])-1)):
#     # plt.plot(file[:,0], i * 2.0 + file[:,i+1], "-", ms=1.5, linewidth=1.25, label="p")
#     plt.plot(file[:-shift,i+1], file[shift:,i+1], "-", ms=1.5, linewidth=1.25, label="p")

plt.legend()
plt.show()