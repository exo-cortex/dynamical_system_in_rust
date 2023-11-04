use crate::network::Edge;

use std::f64::consts::PI;
// use std::fmt::Display;

// 2. with delay
// delay systems must implement
// StateT = state: contains the dynamic variables
// ModelT = model: contains the parameters
// FeedbackT = delay: a delay system has to `keep` some derived quantity of `state` for delayed feedback
// WeightT = weight: the delay will likely be used as feedback in a weighted sum
// KeepT: some object that can be "collected"

// pub trait State:
//     Sized
//     + Clone
//     + Copy
//     + Default
//     + Display
//     + std::iter::Sum
//     + std::ops::Add<Self, Output = Self>
//     + std::ops::AddAssign
//     + std::ops::Mul<f64, Output = Self>
//     + std::ops::Div<f64, Output = Self>
//     + IntoString
// {
// }

pub trait DynamicalSystem {
    type StateT: Sized
        + Clone
        + Copy
        + Default
        // + Display
        + std::ops::Mul<f64, Output = Self::StateT>
        + std::ops::Add<Self::StateT, Output = Self::StateT>
        + std::ops::AddAssign
        + std::ops::Div<f64, Output = Self::StateT>;
    type ModelT: Clone + Copy + Default;
    // fn keep_state(state: &Self::StateT) -> Self::KeepT;
    // type KeepT: Clone + Copy + Default;
    fn keep_state(state: &Self::StateT) -> Vec<f64>;
    fn keep_state_names() -> &'static [&'static str];
}

pub trait Feedback: DynamicalSystem {
    type FeedbackT: Sized
        + Clone
        + Copy
        + Default
        + std::iter::Sum
        + std::ops::Add<Output = Self::FeedbackT>
        + std::ops::AddAssign
        + std::ops::Sub<Output = Self::FeedbackT>
        + std::ops::Mul<f64, Output = Self::FeedbackT>
        + std::ops::Mul<Self::WeightT, Output = Self::FeedbackT>;
    type WeightT: WeightFromEdge
        + Sized
        + Clone
        + Copy
        + Default
        + std::ops::Mul
        + std::ops::Mul<num_complex::Complex<f64>>
        + std::ops::Mul<f64>
        + std::ops::Mul<Self::FeedbackT>;
    fn f(
        state: &Self::StateT,
        model: &Self::ModelT,
        feedback: &Self::FeedbackT,
        // time: &f64, // maybe different ?
    ) -> Self::StateT;
    fn get_feedback(state: &Self::StateT) -> Self::FeedbackT;
    fn keep_state_and_delay(state: &Self::StateT, feedback: &Self::FeedbackT) -> Vec<f64>;
    fn keep_state_and_delay_names() -> &'static [&'static str];
}

pub type WeightReal = f64;
pub type WeightComplex = num_complex::Complex<f64>;

pub trait WeightFromEdge {
    fn from_edge(edge: &Edge) -> Self;
}

impl WeightFromEdge for WeightReal {
    fn from_edge(edge: &Edge) -> Self {
        edge.strength
    }
}

impl WeightFromEdge for WeightComplex {
    fn from_edge(edge: &Edge) -> Self {
        edge.strength * (edge.turn * num_complex::Complex::<f64>::i() * 2.0 * PI).exp()
    }
}

// experimental traits

pub trait InitFunctions {} // todo!()

pub trait AsData<const N: usize> {
    fn get_data(&self) -> [f64; N];
}
