use crate::dynamical_system::{DynamicalSystem, Feedback, WeightReal};
use derive_more::{Add, AddAssign, Div, Mul, MulAssign};

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
        State {
            x: state.y + phi(&state.x, &model) - state.z + delay,
            y: psi(&state.x, &model) - state.y,
            z: model.r * (model.s * (state.x - model.x_r) - state.z),
        }
    }
    fn get_feedback(state: &Self::StateT) -> Self::FeedbackT {
        state.x
    }
    fn keep_state_and_delay(state: &Self::StateT, feedback: &Self::FeedbackT) -> Vec<f64> {
        vec![state.x, state.y, state.z, *feedback]
    }
    fn keep_state_and_delay_names() -> &'static [&'static str] {
        &["x", "y", "z", "x_delay", "y_delay", "z_delay"]
    }
}

fn phi(x: &f64, model: &Model) -> f64 {
    -model.a * x.powi(3) + model.b * x.powi(2)
}

fn psi(x: &f64, model: &Model) -> f64 {
    model.c - model.d * x.powi(2)
}

pub type FeedbackState = f64;

#[derive(Copy, Clone, Add, AddAssign, Mul, MulAssign, Div, Debug)]
pub struct State {
    x: f64,
    y: f64,
    z: f64,
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

#[derive(Copy, Clone)]
pub struct Model {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    r: f64,
    s: f64,
    x_r: f64,
}

impl Default for Model {
    fn default() -> Model {
        Model {
            a: 1.0,
            b: 3.0,
            c: 1.0,
            d: 5.0,
            r: 0.005,
            s: 4.0,
            x_r: 1.6,
        }
    }
}
