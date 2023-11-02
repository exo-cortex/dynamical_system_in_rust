mod calculation;
mod composite_system;
mod dynamical_system;
mod history;
mod integration_methods;
mod lang_kobayashi;
mod mackey_glass;
mod network;

use calculation::{NodeSetup, SystemType};
use network::Network;

// mod curve_simplification;
// use timeseries::Timeseries;
mod hindmarsh_rose;
mod lorenz;
// mod mdre;
mod stuart_landau;

fn main() {
    let inv_dt = 64.0;
    let segment_size = 4096;

    let mut network = Network::new(1, 0.1, 0.1, 100.0, 0, 1.0 / inv_dt);
    network.put_edge(0, 0, 1.0, 0.5, 15.57);
    // network.put_ring(0.01, 0.1, 200.0);

    let mut calculation =
        // calculation::Calculation::example_setup_lang_kobayashi(5, 1.0 / inv_dt, 1024);
        calculation::Calculation::examples(1.0 / inv_dt, &network, segment_size, NodeSetup::Identical, SystemType::MackeyGlass);

    // calculation.n_steps_rk4((1000.0 * inv_dt) as usize);

    for _ in 0..10 {
        calculation.integrate_segment_and_save();
    }

    println!("integrated {} steps", calculation.total_steps);
}
