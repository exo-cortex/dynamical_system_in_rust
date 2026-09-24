use crate::dynamical_system::{DynamicalSystem, Feedback, WeightComplex};
use derive_more::{Add, AddAssign, Div, Mul, MulAssign};
use num_complex::Complex;

pub struct System {}
impl DynamicalSystem for System {
    type StateT = State;
    type ModelT = Model;
    fn keep_state(state: &Self::StateT) -> Vec<f64> {
        vec![state.z.norm_sqr()]
    }
    fn keep_state_names() -> &'static [&'static str] {
        &["z"]
    }
}

impl Feedback for System {
    type FeedbackT = FeedbackState;
    type WeightT = WeightComplex;
    fn f(input_state: &Self::StateT, model: &Model, delay: &Self::FeedbackT) -> Self::StateT {
        Self::StateT {
            z: (Complex::new(model.lambda, model.omega) + model.gamma * input_state.z.norm_sqr())
                * input_state.z
                + delay,
        }
    }
    fn get_feedback(state: &Self::StateT) -> Self::FeedbackT {
        state.z
    }
    fn keep_state_and_delay(state: &Self::StateT, feedback: &Self::FeedbackT) -> Vec<f64> {
        vec![state.z.norm_sqr(), feedback.norm_sqr()]
    }
    fn keep_state_and_delay_names() -> &'static [&'static str] {
        &["z", "z_delay"]
    }
}

pub type FeedbackState = Complex<f64>;

#[derive(Copy, Clone, Add, AddAssign, Mul, MulAssign, Div, Debug)]
pub struct State {
    pub z: Complex<f64>,
}

impl Default for State {
    fn default() -> Self {
        State {
            z: Complex { re: 1.0, im: 0.1 },
        }
    }
}

#[derive(Copy, Clone)]
pub struct Model {
    pub lambda: f64,
    pub omega: f64,
    pub gamma: Complex<f64>,
}
impl Default for Model {
    fn default() -> Self {
        Model {
            lambda: -0.1,
            omega: 1.0,
            gamma: Complex::<f64>::new(-0.1, 1.0),
        }
    }
}
