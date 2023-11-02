# Reservoir computing with dynamical systems in Rust
A programm to integrate dynamical systems written in Rust.

At the moment it is capable of calculating the Lang-Kobayashi- and Mackey-Glass-system.

This Program is mainly for learning Rust. Right now it can only integrate the Lang-Kobayashi- and Mackey-Glass system and then write out the dynamical variables. There's a simple `plot.py` script to see the timeseries.

## implemented features
- ### traits
    - dynamical systems
    - dynamical systems with feedback

- timeseries simplification via RDP[^2] to greatly reduce file sizes
- runge-kutta-4 integration method for delay-differential equations
- dynamical systems: Lang-Kobayashi, Mackey-Glass, Stuart-Landau, Hindmarsh-Rose, Lorenz
- multi-delay network topologies.

## todos
- optimize the perpendicular distance from a point to a line in n dimensions.
- functionality to simplify n-dimensional trajectories into individual one-dimensional time-series
- implement differential equations with delay
### traits-todos
    - dynamical systems with noise
    - driven dynamical systems 

- ### add more dynamical systems
    - Rössler
    - FitzHugh-Nagumo

- ### reservoir computing benchmarks
    - NARMA
    - Legendre-functions
    - prediction of dynamical-systems

[^1]: https://en.wikipedia.org/wiki/Lorenz_system
[^2]: [Ramer-Douglas-Peucker algorithm](https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm)