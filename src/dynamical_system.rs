// idea: define the dynamical system interface
// new dynamical systems must implement it in order to work with the rest

// 1. without delay
pub trait DynamicalSystem<S, M>
where
    S: Sized
        + Copy
        + std::ops::Mul<f64, Output = S>
        + std::ops::Add<S, Output = S>
        + std::ops::AddAssign
        + std::ops::Div<f64, Output = S>,
{
    fn f(state: &S, model: &M) -> S;
}

// 2. with delay
pub trait DynamicalDelaySystem<S, M, D>
where
    S: Sized
        + Copy
        + std::ops::Mul<f64, Output = S>
        + std::ops::Add<S, Output = S>
        + std::ops::AddAssign
        + std::ops::Div<f64, Output = S>,
    D: Sized
        + Clone
        + Copy
        + std::ops::Add<Output = D>
        + std::ops::Sub<Output = D>
        + std::ops::Mul<f64, Output = D>,
{
    fn f(state: &S, model: &M, delay: &D) -> S;
    // from the systems's set of dynamical variables get the subset needed as delay.
    fn keep_delay(state: &S) -> D;
}

// 3. driven dynamical system
// prototype
pub trait DrivenDynamicalDelaySystem<S, M, D, I>
where
    S: Sized
        + Copy
        + std::ops::Mul<f64, Output = S>
        + std::ops::Add<S, Output = S>
        + std::ops::AddAssign
        + std::ops::Div<f64, Output = S>,
    D: Sized
        + Clone
        + Copy
        + std::ops::Add<Output = D>
        + std::ops::Sub<Output = D>
        + std::ops::Mul<f64, Output = D>,
{
    fn f(state: &S, model: &M, delay: &D, input: &I) -> S;
    fn keep_delay(state: &S) -> D;
}
