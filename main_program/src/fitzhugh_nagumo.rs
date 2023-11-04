use crate::dynamical_system::{DynamicalSystem, Feedback, WeightReal};

use derive_more::{Add, AddAssign, Div, Mul, MulAssign};

pub struct System {}
impl DynamicalSystem for System {
    type StateT = State;
    type ModelT = Model;
    fn keep_state(state: &Self::StateT) -> Vec<f64> {
        vec![state.v, state.w]
    }
    fn keep_state_names() -> &'static [&'static str] {
        &["v", "w"]
    }
}

impl Feedback for System {
    type FeedbackT = FeedbackState;
    type WeightT = Weight;
    fn f(state: &Self::StateT, model: &Self::ModelT, delay: &Self::FeedbackT) -> Self::StateT {
        State {
            v: state.v - state.v.powi(3) / 3.0 - state.w + model.i_ext + delay,
            w: (state.v + model.a - model.b * state.w) / model.tau,
        }
    }

    fn get_feedback(state: &Self::StateT) -> Self::FeedbackT {
        state.v
    }
    fn keep_state_and_delay(state: &Self::StateT, feedback: &Self::FeedbackT) -> Vec<f64> {
        vec![state.v, state.w, *feedback]
    }
    fn keep_state_and_delay_names() -> &'static [&'static str] {
        &["x", "y", "z", "x_delay"]
    }
}

type FeedbackState = f64;
type Weight = WeightReal;

#[derive(Copy, Clone, Add, AddAssign, Mul, MulAssign, Div, Debug)]
pub struct State {
    v: f64,
    w: f64,
}

impl Default for State {
    fn default() -> Self {
        State { v: 1.0, w: 1.0 }
    }
}

#[derive(Copy, Clone)]
pub struct Model {
    tau: f64,
    a: f64,
    b: f64,
    i_ext: f64,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            tau: 12.5,
            a: 0.7,
            b: 0.8,
            i_ext: 0.25,
        }
    }
}
