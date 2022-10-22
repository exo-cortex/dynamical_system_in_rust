extern crate derive_more;

use derive_more::{
    Add,
    AddAssign,
    Div,
    Mul,
    MulAssign, // , Sum
};
use num_complex::Complex;

use crate::dynamical_system::{DynamicalDelaySystem, DynamicalSystem};

pub type DelayState = Complex<f64>;
pub type Weight = Complex<f64>;

#[derive(Copy, Clone, Add, AddAssign, Mul, MulAssign, Div, Debug)]
pub struct State {
    pub z: Complex<f64>,
}

pub struct Model {
    pub lambda: f64,
    pub omega: f64,
    pub gamma: Complex<f64>,
}

pub struct System {}
impl DynamicalSystem<State, Model> for System {
    // f as in { z'(t) = f(z(t), t) }
    fn f(input_state: &State, model: &Model) -> State {
        State {
            z: (Complex::new(model.lambda, model.omega) + model.gamma * input_state.z.norm_sqr())
                * input_state.z,
        }
    }
}

pub struct DelaySystem {}
impl DynamicalDelaySystem<State, Model, DelayState> for DelaySystem {
    fn f(input_state: &State, model: &Model, delay: &Complex<f64>) -> State {
        State {
            z: (Complex::new(model.lambda, model.omega) + model.gamma * input_state.z.norm_sqr())
                * input_state.z
                + delay,
        }
    }
    fn keep_delay(state: &State) -> DelayState {
        state.z
    }
}
