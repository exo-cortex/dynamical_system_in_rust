# A Dynamical System in Rust
A programm to integrate dynamical systems written in Rust.

At the moment it is capable of calculating the Lorenz-system[^1] and Lang-Kobayashi-system.

This Program is mainly for learning Rust. Right now it can only integrate the Lorenz system, write out the dynamical variables and plot with the help of a python script.

## implemented features
- timeseries simplification via RDP[^2] to greatly reduce file sizes
- rudimentary integrator module with abstract runge-kutta-4 and euler methods
- dynamical systems: Lorenz, Lang-Kobayashi

## todo:

- optimize calculate the perpendicular distance from a point to a line in n dimensions.
- functionality to simplify n-dimensional trajectories into individual one-dimensional time-series
- implement more dynamical systems
- implement differential equations with delay

[^1]: https://en.wikipedia.org/wiki/Lorenz_system
[^2]: [Ramer-Douglas-Peucker algorithm](https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm)