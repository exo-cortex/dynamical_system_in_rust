#!/usr/bin/env python3

import matplotlib.pyplot as plt
import numpy as np

file_rk4 = np.loadtxt("lorenz_data_rk4.txt")
x = 1 # can be 1,2,3,4
y = 3 # 
plt.plot(file_rk4[:,x], file_rk4[:,y], "-", ms=1.125, linewidth=1.125, label="(x_{}(t), x_{}(t)) runge kutta 4".format(x,y),color=(0,0,0,1.0))

plt.legend()
plt.show()