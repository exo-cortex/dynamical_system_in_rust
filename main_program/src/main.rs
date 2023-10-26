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

fn main() {
    let inv_dt = 64.0;

    let mut network = Network::new(16, 0.1, 0.1, 100.0, 0, 1.0 / inv_dt);
    // network.put_edge(1, 1, 0.1, 0.3, 45.5);
    network.put_edge(0, 0, 0.6, 0.5, 3.21);
    network.put_ring(0.1, 0.2, 20.0);
    // let mut calculation = calculation::Calculation::example_setup_mackey(5, 1.0 / inv_dt, 4096);
    let mut calculation =
        // calculation::Calculation::example_setup_lang_kobayashi(5, 1.0 / inv_dt, 1024);
        calculation::Calculation::examples(1.0 / inv_dt, &network, 512, NodeSetup::Identical ,SystemType::LangKobayashi);

    // calculation.n_steps_rk4((1000.0 * inv_dt) as usize);

    // let mut ts = Timeseries::new(1.0 / inv_dt, 2, 3, 1000, vec!["e", "n"]);

    // calculation.integrate_segment();
    for _ in 0..500 {
        calculation.integrate_segment_and_save();
    }

    // println!("{}", &calculation.timeseries);

    // for _ in 0..5000 {
    //     // this should be done in a single step
    //     for _ in 0..20 {
    //         calculation.single_step_rk4_count();
    //     }
    //     calculation.write_out(&mut writer);
    // }

    println!("integrated {} steps", calculation.total_steps);
}
