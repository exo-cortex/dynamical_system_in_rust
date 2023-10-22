use crate::dynamical_system::{DynamicalSystem, Feedback, IntoString, WeightReal};

use derive_more::{Add, AddAssign, Div, Mul, MulAssign};
use std::fmt;

#[allow(dead_code)]
pub struct System {}
impl DynamicalSystem for System {
    type StateT = State;
    type ModelT = Model;
    type KeepT = Keep;
    fn keep_state(state: &Self::StateT) -> Self::KeepT {
        state.p
    }
}

#[allow(dead_code)]
impl Feedback for System {
    type FeedbackT = FeedbackState;
    type WeightT = WeightReal;
    fn f(state: &Self::StateT, model: &Self::ModelT, delay: &Self::FeedbackT) -> Self::StateT {
        Self::StateT {
            p: model.beta_0 / (1.0 + delay).powi(model.n) - model.gamma * state.p,
        }
    }
    fn get_feedback(state: &Self::StateT) -> FeedbackState {
        state.p
    }
}

pub type FeedbackState = f64;
pub type Keep = f64;

#[allow(dead_code)]
#[derive(Copy, Clone, Add, AddAssign, Mul, MulAssign, Div, Debug, Default)]
pub struct State {
    pub p: f64,
}

impl IntoString for State {
    fn write_out(&self) -> String {
        format!("{}\n", self.p)
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p: {}", self.p)
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Model {
    pub beta_0: f64,
    pub n: i32,
    pub gamma: f64,
}

#[allow(dead_code)]
// from wikipedia
impl Default for Model {
    fn default() -> Model {
        Model {
            beta_0: 0.2,
            n: 10,
            gamma: 0.1,
        }
    }
}
