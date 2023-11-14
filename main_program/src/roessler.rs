// extern crate derive_more;
use crate::dynamical_system::{DynamicalSystem, Feedback, WeightReal};
use derive_more::{Add, AddAssign, Div, Mul, MulAssign};
use std::fmt::{self, Display};

pub struct System {}
impl DynamicalSystem for System {
    type StateT = State;
    type ModelT = Model;
    fn keep_state(state: &Self::StateT) -> Vec<f64> {
        vec![state.x, state.y, state.z]
    }
    fn keep_state_names() -> &'static [&'static str] {
        &["x", "y", "z"]
    }
}

impl Feedback for System {
    type FeedbackT = FeedbackState;
    type WeightT = WeightReal;
    fn f(state: &Self::StateT, model: &Self::ModelT, delay: &Self::FeedbackT) -> Self::StateT {
        Self::StateT {
            x: -state.y - state.z + delay,
            y: state.x + model.a * state.y,
            z: model.b + state.z * (state.x - model.c),
        }
    }
    fn get_feedback(state: &Self::StateT) -> Self::FeedbackT {
        state.x
    }
    fn keep_state_and_delay(state: &Self::StateT, feedback: &Self::FeedbackT) -> Vec<f64> {
        vec![state.x, state.y, state.z, *feedback]
    }
    fn keep_state_and_delay_names() -> &'static [&'static str] {
        &["x", "y", "z", "x_delay"]
    }
}

type FeedbackState = f64;

#[derive(Copy, Clone, Add, AddAssign, Mul, MulAssign, Div, Debug)]
pub struct State {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for State {
    fn default() -> Self {
        State {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "lorenz-system-state: x: {}, y: {}, z: {}",
            self.x, self.y, self.z
        )
    }
}

#[derive(Copy, Clone)]
pub struct Model {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Default for Model {
    fn default() -> Model {
        Model {
            a: 0.2,
            b: 0.2,
            c: 5.7,
        }
    }
}
