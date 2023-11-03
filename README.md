# Reservoir computing with dynamical systems in Rust
A programm to integrate dynamical systems written in Rust.

At the moment it is capable of calculating the Lang-Kobayashi- and Mackey-Glass-system.

This Program is mainly for learning Rust. Right now it can only integrate the Lang-Kobayashi- and Mackey-Glass system and then write out the dynamical variables. There's a simple `plot.py` script to see the timeseries.

## implemented features
- timeseries simplification via RDP[^2] to greatly reduce file sizes
- runge-kutta-4 integration method for delay-differential equations
- dynamical systems: Lang-Kobayashi, Mackey-Glass, Stuart-Landau, Hindmarsh-Rose, Lorenz
- multi-delay network topologies.

### traits
- dynamical systems
- dynamical systems with feedback

## todos
- ### add more dynamical systems
    - Rössler
    - FitzHugh-Nagumo
    - Van der Pol oscillator

- ### reservoir computing benchmarks
    - NARMA
    - Legendre-functions
    - prediction of dynamical-systems

- optimize the perpendicular distance from a point to a line in n dimensions.
- functionality to simplify n-dimensional trajectories into individual one-dimensional time-series
- implement differential equations with delay

### traits
    - implement generalized `FeedbackType`-trait to allow each system define a specific type of feedback.
    - implement generalized `Weight`-trait
        right now only two types for delayed-Feedback are possible : `Complex<f64>` and `f64`
        but systems might want `[f64; 3]` as Feedback-type and something similar to a matrix as weight. 
        That way each system can have each feedback-input be a weighted sum of the state's variables.
    - systems with noise
    - systems with external driving force (needed for reservoir computing)

[^1]: https://en.wikipedia.org/wiki/Lorenz_system
[^2]: [Ramer-Douglas-Peucker algorithm](https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm)