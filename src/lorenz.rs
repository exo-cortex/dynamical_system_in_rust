extern crate derive_more;
use derive_more::{Add, AddAssign, Div, Mul, MulAssign};

#[derive(Copy, Clone, Add, AddAssign, Mul, MulAssign, Div, Debug)]
pub struct State {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Model {
    pub sigma: f64,
    pub beta: f64,
    pub rho: f64,
}

#[allow(dead_code)]
impl Model {
    pub fn ff(&self, input: &State) -> State {
        State {
            x: self.sigma * (input.y - input.x),
            y: input.x * (self.rho - input.z) - input.y,
            z: input.x * input.y - self.beta * input.z,
        }
    }
}

pub fn f(input: &State, p: &Model) -> State {
    // f as in { z'(t) = f(z(t), t) }
    State {
        x: p.sigma * (input.y - input.x),
        y: input.x * (p.rho - input.z) - input.y,
        z: input.x * input.y - p.beta * input.z,
    }
}

#[allow(dead_code)]
pub fn update_rk4(state: &mut State, p: &Model, dt: f64) {
    // runge kutta 4 method creates 4 "helper steps"
    let k1 = f(state, p);
    let k2 = f(&(*state + k1 * 0.5 * dt), p);
    let k3 = f(&(*state + k2 * 0.5 * dt), p);
    let k4 = f(&(*state + k3 * dt), p);

    *state += (k1 + k2 * 2.0 + k3 * 2.0 + k4) / 6.0 * dt;
}
