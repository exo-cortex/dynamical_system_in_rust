use crate::dynamical_system::{AsData, DynamicalSystem, Feedback, WeightComplex};
use derive_more::{Add, AddAssign, Div, Mul, MulAssign};
use num_complex::Complex;
use std::fmt;

// const DOMAIN_NAME: &'static str = "lang_kobayashi";

// impl DynamicalSystem for System {
//     type StateT = State;
//     type ModelT = ModelT;
//     type KeepT = Vec<f64>;
//     fn f(input_state: &Self::StateT, model: &Self::ModelT) -> Self::StateT {
//         Self::StateT {
//             e: Complex::new(1.0, model.alpha) * input_state.n * input_state.e,
//             n: (1.0 / model.t_lk)
//                 * (model.pump
//                     - input_state.n
//                     - (2.0 * input_state.n + 1.0) * input_state.e.norm_sqr()),
//         }
//     }
//     fn collect(state: &Self::StateT) -> Self::KeepT {
//         vec![state.e.norm_sqr(), state.n]
//     }
// }

#[allow(dead_code)]
pub struct System {}
impl DynamicalSystem for System {
    type StateT = State;
    type ModelT = Model;
    fn keep_state(state: &Self::StateT) -> Vec<f64> {
        vec![state.e.norm_sqr(), state.n]
    }
    fn keep_state_names() -> &'static [&'static str] {
        &["e_norm", "n"]
    }
}

#[allow(dead_code)]
impl Feedback for System {
    type FeedbackT = FeedbackState;
    type WeightT = WeightComplex;
    fn f(state: &Self::StateT, model: &Self::ModelT, delay: &Self::FeedbackT) -> Self::StateT {
        Self::StateT {
            e: Complex::new(1.0, model.alpha) * state.n * state.e + delay,
            n: (1.0 / model.t_lk)
                * (model.pump - state.n - (2.0 * state.n + 1.0) * state.e.norm_sqr()),
        }
    }
    fn get_feedback(state: &Self::StateT) -> Self::FeedbackT {
        state.e
    }
    fn keep_state_and_delay(state: &Self::StateT, feedback: &Self::FeedbackT) -> Vec<f64> {
        vec![state.e.re, state.e.im, state.n, feedback.re, feedback.im]
    }
    fn keep_state_and_delay_names() -> &'static [&'static str] {
        &["e_real", "e_imag", "n", "e_delay_im", "e_delay_re"]
    }
}

pub type FeedbackState = Complex<f64>;

#[allow(dead_code)]
#[derive(Copy, Clone, Add, AddAssign, Mul, MulAssign, Div, Debug)]
pub struct State {
    pub e: Complex<f64>,
    pub n: f64,
}

impl Default for State {
    fn default() -> Self {
        State {
            e: Complex::<f64>::new(0.1, 0.0),
            n: 0.05,
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "lang-kobayashi-state: Re(e): {}, Im(e): {}, n: {}",
            self.e.re, self.e.im, self.n
        )
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Model {
    pub alpha: f64,
    pub pump: f64,
    pub t_lk: f64,
}

#[allow(dead_code)]
impl Default for Model {
    fn default() -> Model {
        Model {
            alpha: 1.5,
            pump: 0.1,
            t_lk: 100.0,
        }
    }
}

// experimental
// AsOutput<N, M> uses
// N - numbering of the output
//     each AsOutput impl has to have a different N
// M - the size of the output array (better fixed-sized array for compilation)
struct AsA<'a>(&'a State);
impl<'a> AsData<2> for AsA<'a> {
    fn get_data(&self) -> [f64; 2] {
        [self.0.e.norm_sqr(), self.0.n]
    }
}

struct AsB<'a>(&'a State);
impl<'a> AsData<3> for AsB<'a> {
    fn get_data(&self) -> [f64; 3] {
        [self.0.e.re, self.0.e.im, self.0.n]
    }
}
