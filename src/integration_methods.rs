use derive_more::{Add, AddAssign, Mul, Sum};

use crate::dynamical_system::{DynamicalSystem, Feedback};

pub trait Integrate: DynamicalSystem {
    fn single_step_rk4(&mut self);
    // fn single_step_rk4_keep(&mut self) -> Self::KeepT;
    fn n_steps_rk4(&mut self, n: usize);
}

// do i need this?
pub trait IntegrateWithFeedback: Feedback {
    fn single_step_rk4(&mut self) -> Self::FeedbackT;
}

pub trait IntegrateAndWrite {
    type KeepT;
    fn keep(&self) -> Self::KeepT;
    // fn integrate_and_write_segment(&mut self);
}

//

pub trait IntegrationMethods {
    fn single_step_rk4(&mut self);
    fn n_steps_rk4(&mut self, n: usize);
    fn into_str(&self) -> String;
}

#[allow(dead_code)]
pub fn single_step_rk4<DynSystemT>(dynamical_system: &mut impl IntegrationMethods)
where
    DynSystemT: Feedback,
{
    dynamical_system.single_step_rk4();
}

#[allow(dead_code)]
pub fn rk4<S>(
    state: &mut S::StateT,
    model: &S::ModelT,
    dt: &f64,
    f: fn(&S::StateT, &S::ModelT) -> S::StateT,
) where
    S: DynamicalSystem,
{
    // runge kutta 4 method creates 4 "helper steps"
    let k1 = f(state, model);
    let k2 = f(&(*state + k1 * 0.5 * *dt), model);
    let k3 = f(&(*state + k2 * 0.5 * *dt), model);
    let k4 = f(&(*state + k3 * *dt), model);

    *state += (k1 + k2 * 2.0 + k3 * 2.0 + k4) / 6.0 * *dt;
}

#[derive(Clone, Copy, Default, Mul, Add, AddAssign, Sum)]
pub struct RungeKuttaDelay<T>
where
    T: Sized
        + Clone
        + Copy
        + Default
        + core::iter::Sum
        + std::ops::Add<Output = T>
        + std::ops::AddAssign,
{
    pub state: T,
    pub slope: T,
}

#[allow(dead_code)]
pub fn rk4_delay<S>(
    state: &mut S::StateT,
    model: &S::ModelT,
    keep_state: &mut RungeKuttaDelay<S::FeedbackT>,
    // with rk4 the delays for the 3 different time_positions
    // at k1, (k2+k3) and k4 are calculated through hermite interpolations
    delay: &[RungeKuttaDelay<S::FeedbackT>; 2],
    dt: &f64,
    f: fn(&S::StateT, &S::ModelT, &S::FeedbackT) -> S::StateT,
    d: fn(&S::StateT) -> S::FeedbackT,
) where
    S: DynamicalSystem + Feedback,
{
    let k1 = f(state, model, &delay[0].state);
    // the middle steps k2,k3 need a delay value not existing in the history. it is created through hermite interpolation.
    let middle =
        (delay[0].state + delay[1].state) * 0.5 + (delay[0].slope - delay[1].slope) * 0.125;
    let k2 = f(&(*state + k1 * 0.5 * *dt), model, &middle);
    let k3 = f(&(*state + k2 * 0.5 * *dt), model, &middle);
    let k4 = f(&(*state + k3 * *dt), model, &delay[1].state);

    *state += (k1 + k2 * 2.0 + k3 * 2.0 + k4) / 6.0 * *dt;
    keep_state.state = d(state);
    keep_state.slope = d(&k1);
}

#[allow(dead_code)]
pub fn euler<S>(
    state: &mut S::StateT,
    model: &S::ModelT,
    dt: &f64,
    f: fn(&S::StateT, &S::ModelT) -> S::StateT,
) where
    S: DynamicalSystem,
{
    *state += f(state, model) * *dt;
}

#[allow(dead_code)]
pub fn euler_delay<S>(
    state: &mut S::StateT,
    model: &S::ModelT,
    delay: &S::FeedbackT,
    dt: &f64,
    f: fn(&S::StateT, &S::ModelT, &S::FeedbackT) -> S::StateT,
) where
    S: DynamicalSystem + Feedback,
{
    *state += f(state, model, delay) * *dt;
}
