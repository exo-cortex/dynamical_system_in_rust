use crate::dynamical_system::{DynamicalSystem, Feedback, IntoString, WeightReal};

use derive_more::{Add, AddAssign, Div, Mul, MulAssign};
use std::fmt;

#[allow(dead_code)]
pub struct System {}
impl DynamicalSystem for System {
    type StateT = State;
    type ModelT = Model;
    fn keep_state(state: &Self::StateT) -> Vec<f64> {
        vec![state.q]
    }
    fn keep_state_names() -> &'static [&'static str] {
        &["p"]
    }
}

#[allow(dead_code)]
impl Feedback for System {
    type FeedbackT = FeedbackState;
    type WeightT = WeightReal;
    fn f(state: &Self::StateT, model: &Self::ModelT, delay: &Self::FeedbackT) -> Self::StateT {
        Self::StateT {
            q: (model.beta_0 * delay) / (1.0 + delay.powi(model.n)) - model.gamma * state.q,
        }
    }
    fn get_feedback(state: &Self::StateT) -> FeedbackState {
        state.q
    }
    fn keep_state_and_delay(state: &Self::StateT, feedback: &Self::FeedbackT) -> Vec<f64> {
        vec![state.q, *feedback]
    }
    fn keep_state_and_delay_names() -> &'static [&'static str] {
        &["q", "q_delay"]
    }
}

pub type FeedbackState = f64;

#[allow(dead_code)]
#[derive(Copy, Clone, Add, AddAssign, Mul, MulAssign, Div, Debug)]
pub struct State {
    pub q: f64,
}

impl Default for State {
    fn default() -> Self {
        State { q: 0.5 }
    }
}

impl IntoString for State {
    fn write_out(&self) -> String {
        format!("{}\t", self.q)
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "q: {}", self.q)
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
