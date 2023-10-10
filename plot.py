#!/usr/bin/env python3

import matplotlib.pyplot as plt
import numpy as np

file = np.loadtxt("test.txt")

for i in range( (len(file[0])-1) // 2):
    plt.plot(file[:,0], file[:,i*2+1], "-", ms=1.5, linewidth=1.25, label="e")
    plt.plot(file[:,0], file[:,i*2+2], "-", ms=1.5, linewidth=1.25, label="n")

plt.legend()
plt.show()