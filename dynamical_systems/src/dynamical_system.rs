use rand::rngs::SmallRng;

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

pub trait UncoupledSystem: DynamicalSystem {
    fn f(
        state: &Self::StateT,
        model: &Self::ModelT,
        time: &f64, // maybe different ?
    ) -> Self::StateT;
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
    type WeightT: Sized
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

// experimental traits
// not yet used

pub trait NoisySystem: DynamicalSystem {}
pub trait DrivenSystem: DynamicalSystem {}

pub trait Init: DynamicalSystem + Feedback {
    fn init_state(
        init_string: &str,
        rng: &mut SmallRng,
        nodes: usize,
        node_index: usize,
    ) -> Self::StateT;
    fn init_feedback(
        init_string: &str,
        rng: &mut SmallRng,
        nodes: usize,
        node_index: usize,
    ) -> Self::FeedbackT;
    fn init_model(
        init_string: &str,
        rng: &mut SmallRng,
        nodes: usize,
        node_index: usize,
    ) -> Self::ModelT;
} // todo!()

// trait to make possible to return data from a system's dynamical
// state in different ways
pub trait AsData {
    fn get_data_descriptions() -> Vec<&'static str>;
    fn get_data(&self) -> Vec<f64>;
}
