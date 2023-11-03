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

mod fitzhugh_nagumo;

fn main() {
    let inv_dt = 4.0 * 64.0;
    let segment_size = 4096;

    let mut network = Network::new(5, 0.1, 0.1, 100.0, 0, 1.0 / inv_dt);
    network.put_edge(0, 0, 0.5, 0.5, 115.57);
    network.put_edge(0, 0, 0.25, 0.5, 51.57);
    network.put_edge(0, 0, 0.125, 0.5, 215.57);
    network.put_ring(0.05, 0.1, 40.0);

    let mut calculation =
        // calculation::Calculation::example_setup_lang_kobayashi(5, 1.0 / inv_dt, 1024);
        calculation::Calculation::examples(1.0 / inv_dt, &network, segment_size, NodeSetup::Identical, SystemType::HindmarshRose);

    // calculation.n_steps_rk4((1000.0 * inv_dt) as usize);

    for _ in 0..20 {
        calculation.integrate_segment_and_save();
    }

    println!("integrated {} steps", calculation.total_steps);
}
