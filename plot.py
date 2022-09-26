#!/usr/bin/env python3

import matplotlib.pyplot as plt
import numpy as np

file_rk4 = np.loadtxt("lorenz_data_rk4.txt")

# timesieries plots
x = 0 # can be 1,2,3,4
plt.plot(file_rk4[:,x], file_rk4[:,1], "-", ms=3.5, linewidth=1.125, label="x_{}(t)".format(1),color=(1,0,0,0.5))
plt.plot(file_rk4[:,x], file_rk4[:,2], "-", ms=3.5, linewidth=1.125, label="x_{}(t)".format(2),color=(0,0.75,0,0.5))
plt.plot(file_rk4[:,x], file_rk4[:,3], "-", ms=3.5, linewidth=1.125, label="x_{}(t)".format(3),color=(0,0,1,0.5))


# # use this for parametric plotting
# x, y = 1, 3
# plt.plot(file_rk4[:,x], file_rk4[:,y], "-", ms=1.25, linewidth=1.125, label="(x_{}(t), x_{}(t)) runge kutta 4".format(x,y),color=(0,0,0,0.5))
# plt.plot(file_rk4[:,x], file_rk4[:,y], ".", ms=1.5, linewidth=1.125, color=(1,0,0,0.5))

plt.legend()
plt.show()