extern crate derive_more;

use derive_more::{Add, AddAssign, Div, Mul, MulAssign};
use num_complex::Complex;

#[derive(Copy, Clone, Add, AddAssign, Mul, MulAssign, Div, Debug)]
pub struct State {
    pub e: Complex<f64>,
    pub n: f64,
}

pub struct Model {
    pub alpha: f64,
    pub pump: f64,
    pub t_lk: f64,
    // pub kappa: f64,
    // pub phi: f64,
}

#[allow(dead_code)]
impl Model {
    pub fn ff(&self, input: &State) -> State {
        State {
            e: Complex::new(1.0, self.alpha) * input.n * input.e,
            n: (1.0 / self.t_lk)
                * (self.pump - input.n - (2.0 * input.n + 1.0) * input.e.norm_sqr()),
        }
    }

    // pub fn update_rk4(&mut self) {
    //     pub fn update_rk4_with_f(f: fn(&State) -> State, state: &mut State, dt: f64) {
    //         // runge kutta 4 method creates 4 "helper steps"
    //         let k1 = f(state);
    //         let k2 = f(&(*state + k1 * 0.5 * dt));
    //         let k3 = f(&(*state + k2 * 0.5 * dt));
    //         let k4 = f(&(*state + k3 * dt));
    //         *state += (k1 + k2 * 2.0 + k3 * 2.0 + k4) / 6.0 * dt;
    //     }
    // }
}

pub fn f(input: &State, p: &Model) -> State {
    // f as in { z'(t) = f(z(t), t) }
    State {
        e: Complex::new(1.0, p.alpha) * input.n * input.e,
        n: (1.0 / p.t_lk) * (p.pump - input.n - (2.0 * input.n + 1.0) * input.e.norm_sqr()),
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
