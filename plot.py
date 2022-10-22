#!/usr/bin/env python3

import matplotlib.pyplot as plt
import numpy as np
import glob

# plt.rcParams.update({
#     "text.usetex": True,
#     "font.family": "sans-serif",
#     "font.sans-serif": "Helvetica",
#     "font.size" : 18,
# })

colors = plt.rcParams["axes.prop_cycle"]()

filenames = glob.glob("data_*.txt")
print(filenames)
files = [np.loadtxt(filename) for filename in filenames]

for i, f in enumerate(files):
    print("data_dimensions: {}".format(np.shape(f)))
    c = next(colors)["color"]
    plt.plot(f[:,0], f[:,1] + i * 3, "-", ms=3.5, linewidth=1.25, color=c , label="e_{}".format(i))

plt.legend()
plt.show()