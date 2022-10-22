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

pub struct System {}
impl DynamicalSystem<State, Model> for System {
    fn f(input_state: &State, model: &Model) -> State {
        State {
            e: Complex::new(1.0, model.alpha) * input_state.n * input_state.e,
            n: (1.0 / model.t_lk)
                * (model.pump
                    - input_state.n
                    - (2.0 * input_state.n + 1.0) * input_state.e.norm_sqr()),
        }
    }
}

pub struct DelaySystem {}
impl DynamicalDelaySystem<State, Model, DelayState> for DelaySystem {
    fn f(input_state: &State, model: &Model, delay: &DelayState) -> State {
        State {
            e: Complex::new(1.0, model.alpha) * input_state.n * input_state.e + delay,
            n: (1.0 / model.t_lk)
                * (model.pump
                    - input_state.n
                    - (2.0 * input_state.n + 1.0) * input_state.e.norm_sqr()),
        }
    }
    fn keep_delay(state: &State) -> DelayState {
        state.e
    }
}

#[derive(Copy, Clone, Default)]
pub struct Model {
    pub alpha: f64,
    pub pump: f64,
    pub t_lk: f64,
}

#[derive(Copy, Clone, Add, AddAssign, Mul, MulAssign, Div, Debug, Default)]
pub struct State {
    pub e: Complex<f64>,
    pub n: f64,
}
