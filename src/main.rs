use std::{
    fs::File,
    io::{BufWriter, Write},
};

mod composite_system;
mod dynamical_system;
// mod global_parameter_map;
mod history;
mod integration_methods;
mod integrator;
mod lang_kobayashi;
mod mackey_glass;
mod network;
// mod parameter;
// mod var;

mod curve_simplification;

// use crate::parameter::Parameter;
// use crate::network::Network;

fn main() {
    let mut file = File::create("./test.txt").unwrap();
    let mut writer = BufWriter::new(file);

    let mut calculation = integrator::Calculation::example_setup(4);

    calculation.n_steps_rk4(100);

    for _ in 0..10000 {
        // this should be done in a single step
        calculation.single_step_rk4();
        calculation.write_out(&mut writer);
    }
}
