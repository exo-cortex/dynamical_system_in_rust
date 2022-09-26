# A Dynamical System in Rust
A programm to integrate dynamical systems written in Rust.

At the moment it is only capable of calculating the Lorenz-System[^1].

This Program is mainly for learning Rust. Right now it can only integrate the Lorenz system, write out the dynamical variables and plot with the help of a python script.

Later it might be used to integrate different dynamical systems and analyse the results on its own.

# implemented features
- timeseries simplification via RDP[^2] to greatly reduce file sizes


# todo:

- optimize calculate the perpendicular distance from a point to a line in n dimensions.
- functionality to simplify n-dimensional trajectories into individual one-dimensional time-series

[^1]: https://en.wikipedia.org/wiki/Lorenz_system
[^2]: Ramer-Douglas-Peucker algorithm