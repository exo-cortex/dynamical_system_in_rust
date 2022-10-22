extern crate derive_more;
use derive_more::{Add, AddAssign, Div, Mul, MulAssign};

use crate::dynamical_system::DynamicalSystem;

// pub type DelayState = f64;
// pub type Weight = f64;

pub struct System {}
impl DynamicalSystem<State, Model> for System {
    fn f(state: &State, model: &Model) -> State {
        State {
            x: model.sigma * (state.y - state.x),
            y: state.x * (model.rho - state.z) - state.y,
            z: state.x * state.y - model.beta * state.z,
        }
    }
}

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
