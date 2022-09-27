pub fn update_runge_kutte_4<T, P>(state: &mut T, parameters: &P, dt: f64, f: fn(&T, &P) -> T)
where
    T: Sized
        + Copy
        + std::ops::Mul<f64, Output = T>
        + std::ops::Add<T, Output = T>
        + std::ops::AddAssign
        + std::ops::Div<f64, Output = T>,
{
    // runge kutta 4 method creates 4 "helper steps"
    let k1 = f(state, &parameters);
    let k2 = f(&(*state + k1 * 0.5 * dt), &parameters);
    let k3 = f(&(*state + k2 * 0.5 * dt), &parameters);
    let k4 = f(&(*state + k3 * dt), &parameters);

    *state += (k1 + k2 * 2.0 + k3 * 2.0 + k4) / 6.0 * dt;
}

#[allow(dead_code)]
pub fn update_euler<T, P>(state: &mut T, parameters: &P, dt: f64, f: fn(&T, &P) -> T)
where
    T: Sized
        + Copy
        + std::ops::Mul<f64, Output = T>
        + std::ops::Add<T, Output = T>
        + std::ops::AddAssign
        + std::ops::Div<f64, Output = T>,
{
    *state += f(state, &parameters) * dt;
}
