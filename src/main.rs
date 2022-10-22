// mod analysis;
mod curve_simplification;
mod dynamical_system;
mod history;
mod integrator;
mod lang_kobayashi;
mod network;

fn main() -> std::io::Result<()> {
    let integration_time = 5000.0;
    let stepsize = 1.0 / 64.0;
    let pre_integration_buffer = 0.0;
    let nodes = 4;

    let mut integrator = integrator::Integrator::new(
        integration_time,
        stepsize,
        pre_integration_buffer,
        4 * 4096,
        nodes,
        None,
    );

    integrator.integrate_rk4();

    Ok(())
}
