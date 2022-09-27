#!/usr/bin/env python3

import matplotlib.pyplot as plt
import numpy as np

file_rk4 = np.loadtxt("data.txt")

# timesieries plots
t = 0 # can be 1,2,3,4

fig, ax1 = plt.subplots()
ax1color=(1,0,0,0.5)
ax1.plot(file_rk4[:,t], file_rk4[:,1], "-", ms=3.5, linewidth=1.25, color=ax1color)
ax1.set_ylabel("|e|(t)", color=ax1color)
ax1.tick_params(axis='y', labelcolor=ax1color)
ax2 = ax1.twinx()
ax2color=(0,0,1,0.5)
ax2.plot(file_rk4[:,t], file_rk4[:,2], "-", ms=3.5, linewidth=1.25, color=ax2color)
ax2.set_ylabel("n(t)", color = ax2color)
ax2.tick_params(axis='y', labelcolor=ax2color)

plt.title("{} data points".format(np.shape(file_rk4)[0]))
plt.show()